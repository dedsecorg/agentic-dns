use std::collections::HashMap;
use std::io::BufRead;
use std::net::SocketAddr;
use std::process::Command;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::time::{interval, Duration};

/// Unified DNS routing server
#[derive(Parser, Debug)]
#[command(name = "agentic-dns-server", version, about)]
struct Args {
    #[arg(short, long, default_value = "/etc/agentic-dns/config.toml")]
    config: String,
    #[arg(short, long, default_value_t = 8099)]
    api_port: u16,
    #[arg(long)]
    monitor: bool,
    #[arg(long)]
    mcp: bool,
    #[arg(long)]
    dot_proxy: bool,
    #[arg(long, default_value = "/etc/agentic-dns/certs/dot.crt")]
    cert_file: String,
    #[arg(long, default_value = "/etc/agentic-dns/certs/dot.key")]
    key_file: String,
    #[arg(long, default_value = "127.0.0.1")]
    upstream_dns: String,
    #[arg(long, default_value_t = 853)]
    dot_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DnsService {
    name: String,
    port: u16,
    addr: String,
    #[serde(rename = "type")]
    svc_type: String,
    upstream: Option<String>,
    fallback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppConfig {
    pihole_host: String,
    pihole_api_key: String,
    protonvpn_dns: String,
    services: Vec<DnsService>,
    alert_webhook: Option<String>,
    health_check_interval_secs: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            pihole_host: "127.0.0.1".to_string(),
            pihole_api_key: "".to_string(),
            protonvpn_dns: "127.0.0.1".to_string(),
            services: vec![
                DnsService { name: "pihole".into(), port: 53, addr: "0.0.0.0".into(), svc_type: "forwarder".into(), upstream: Some("coredns".into()), fallback: None },
                DnsService { name: "coredns".into(), port: 5352, addr: "127.0.0.1".into(), svc_type: "split-horizon".into(), upstream: Some("dnsdist".into()), fallback: None },
                DnsService { name: "dnsdist".into(), port: 5330, addr: "127.0.0.1".into(), svc_type: "loadbalancer".into(), upstream: Some("unbound,dnscrypt,stubby".into()), fallback: None },
                DnsService { name: "unbound".into(), port: 5335, addr: "127.0.0.1".into(), svc_type: "resolver".into(), upstream: Some("protonvpn-dns".into()), fallback: None },
                DnsService { name: "stubby".into(), port: 5360, addr: "127.0.0.1".into(), svc_type: "dot-proxy".into(), upstream: Some("protonvpn-dns".into()), fallback: Some("unbound".into()) },
                DnsService { name: "dnscrypt".into(), port: 5354, addr: "127.0.0.1".into(), svc_type: "doh-proxy".into(), upstream: Some("cloudflare-doh".into()), fallback: None },
                DnsService { name: "protonvpn-dns".into(), port: 53, addr: "127.0.0.1".into(), svc_type: "upstream".into(), upstream: None, fallback: None },
            ],
            alert_webhook: None,
            health_check_interval_secs: 5,
        }
    }
}

fn load_config(path: &str) -> AppConfig {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            match toml::from_str::<AppConfig>(&content) {
                Ok(cfg) => cfg,
                Err(_) => {
                    eprintln!("Warning: config parse error, using defaults");
                    AppConfig::default()
                }
            }
        }
        Err(_) => {
            eprintln!("Warning: config file {} not found, using defaults", path);
            AppConfig::default()
        }
    }
}

fn is_port_listening(port: u16) -> bool {
    let output = Command::new("ss").args(["-tulnp"]).output();
    match output {
        Ok(out) => ss_output_has_port(&String::from_utf8_lossy(&out.stdout), port),
        Err(_) => false,
    }
}

fn ss_output_has_port(ss_output: &str, port: u16) -> bool {
    // Column 4 is Local Address:Port; the port is everything after the last ':'
    // so bracketed/unbracketed IPv6 addresses are handled.
    ss_output.lines().skip(1).any(|line| {
        line.split_whitespace()
            .nth(4)
            .and_then(|local| local.rsplit_once(':'))
            .and_then(|(_, p)| p.parse::<u16>().ok())
            == Some(port)
    })
}

#[cfg(test)]
mod tests {
    use super::ss_output_has_port;

    const SS: &str = "\
Netid State  Recv-Q Send-Q Local Address:Port  Peer Address:PortProcess
udp   UNCONN 0      0         127.0.0.1:5330       0.0.0.0:*    users:((\"dnsdist\",pid=1,fd=5))
udp   UNCONN 0      0         127.0.0.1:5352       0.0.0.0:*
tcp   LISTEN 0      4096           [::]:5354          [::]:*
tcp   LISTEN 0      4096              *:853             *:*
garbage line without enough columns
";

    #[test]
    fn exact_port_only() {
        assert!(!ss_output_has_port(SS, 53));
        assert!(ss_output_has_port(SS, 5330));
        assert!(ss_output_has_port(SS, 5352));
    }

    #[test]
    fn ipv6_and_wildcard_addresses() {
        assert!(ss_output_has_port(SS, 5354));
        assert!(ss_output_has_port(SS, 853));
    }

    #[test]
    fn header_and_malformed_rows_ignored() {
        assert!(!ss_output_has_port(SS, 0));
        assert!(!ss_output_has_port("", 53));
        assert!(!ss_output_has_port("Netid State Recv-Q Send-Q Local Address:Port\n", 53));
    }
}

fn dns_query(domain: &str, addr: &str, port: u16) -> Option<String> {
    let host = if addr == "0.0.0.0" { "127.0.0.1" } else { addr };
    let output = Command::new("dig")
        .args(["+short", domain, &format!("@{}", host), "-p", &port.to_string()])
        .output();
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let result = stdout.trim().to_string();
            if result.is_empty() { None } else { Some(result) }
        }
        Err(_) => None,
    }
}

fn check_service(svc: &DnsService) -> (bool, bool) {
    let listening = is_port_listening(svc.port);
    let responsive = if listening {
        dns_query("example.com", &svc.addr, svc.port).is_some()
    } else {
        false
    };
    (listening, responsive)
}

fn discover_services(config: &AppConfig) -> Vec<Value> {
    let mut results = vec![];
    for svc in &config.services {
        let (listening, responsive) = check_service(svc);
        let status = if !listening { "down" } else if !responsive { "unhealthy" } else { "up" };
        results.push(json!({
            "name": svc.name,
            "address": format!("{}:{}", svc.addr, svc.port),
            "type": svc.svc_type,
            "status": status,
            "listening": listening,
            "responsive": responsive,
            "upstream": svc.upstream,
        }));
    }
    results
}

fn get_chain() -> Vec<String> {
    vec![
        "pihole (:53) - ad/tracker blocking".into(),
        "coredns (:5352) - split-horizon mesh DNS".into(),
        "dnsdist (:5330) - load balancer across all upstreams".into(),
        "unbound (:5335) - recursive resolver + DNSSEC".into(),
        "dnscrypt-proxy (:5354) - DoH to Cloudflare/Quad9".into(),
        "stubby (:5360) - DoT to upstreams".into(),
        "protonvpn-dns (127.0.0.1) - final upstream via proton0".into(),
    ]
}

// ---- API Handlers ----
async fn api_status(Query(_): Query<HashMap<String, String>>) -> (StatusCode, Json<Value>) {
    let config = load_config("/etc/agentic-dns/config.toml");
    let services = discover_services(&config);
    let chain = get_chain();
    (StatusCode::OK, Json(json!({
        "services": services,
        "chain": chain,
        "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
    })))
}

async fn api_health(Query(_): Query<HashMap<String, String>>) -> (StatusCode, Json<Value>) {
    let config = load_config("/etc/agentic-dns/config.toml");
    let services = discover_services(&config);
    let all_healthy = services.iter().all(|s| s["status"] == "up");
    (StatusCode::OK, Json(json!({
        "healthy": all_healthy,
        "services": services.iter().map(|s| json!({
            "name": &s["name"],
            "status": &s["status"],
            "listening": &s["listening"],
            "responsive": &s["responsive"],
        })).collect::<Vec<_>>(),
    })))
}

async fn api_query(Query(params): Query<HashMap<String, String>>) -> (StatusCode, Json<Value>) {
    let domain = params.get("domain").cloned().unwrap_or_else(|| "example.com".to_string());
    let addr = params.get("addr").cloned().unwrap_or_else(|| "127.0.0.1".to_string());
    let port = params.get("port").cloned().unwrap_or_else(|| "53".to_string());
    let result = dns_query(&domain, &addr, port.parse().unwrap_or(53));
    (StatusCode::OK, Json(json!({
        "domain": domain,
        "resolver": format!("{}:{}", addr, port),
        "result": result,
    })))
}

async fn api_trace(Query(params): Query<HashMap<String, String>>) -> (StatusCode, Json<Value>) {
    let domain = params.get("domain").cloned().unwrap_or_else(|| "example.com".to_string());
    let config = load_config("/etc/agentic-dns/config.toml");
    let services = discover_services(&config);
    let chain = get_chain();
    let result = dns_query(&domain, "127.0.0.1", 53);
    (StatusCode::OK, Json(json!({
        "domain": domain,
        "chain": chain,
        "services": services,
        "result": result,
    })))
}

async fn api_pihole(Query(_): Query<HashMap<String, String>>) -> (StatusCode, Json<Value>) {
    let config = load_config("/etc/agentic-dns/config.toml");
    // PiHole v6 API is minimal; read directly from FTL SQLite database
    let db_path = "/etc/pihole/pihole-FTL.db";

    // Get recent queries
    let recent = Command::new("sqlite3")
        .args(["-json", db_path, &format!(
            "SELECT datetime(qs.timestamp, 'unixepoch') as timestamp, \
             CASE qs.type WHEN 1 THEN 'A' WHEN 2 THEN 'AAAA' ELSE 'OTHER' END as qtype, \
             d.domain as domain, \
             CASE qs.status WHEN 0 THEN 'blocked' WHEN 1 THEN 'gravity' WHEN 2 THEN 'cache' \
               WHEN 3 THEN 'forward' WHEN 9 THEN 'blocked' ELSE 'other' END as status, \
             c.ip as client_ip \
             FROM query_storage qs \
             LEFT JOIN domain_by_id d ON qs.domain = d.id \
             LEFT JOIN client_by_id c ON qs.client = c.id \
             ORDER BY qs.id DESC LIMIT 20"
        )])
        .output();

    // Get top domains
    let top = Command::new("sqlite3")
        .args(["-json", db_path, &format!(
            "SELECT d.domain as domain, COUNT(*) as count FROM query_storage qs \
             LEFT JOIN domain_by_id d ON qs.domain = d.id \
             GROUP BY d.domain ORDER BY count DESC LIMIT 10"
        )])
        .output();

    // Get summary stats
    let stats = Command::new("sqlite3")
        .args([db_path, "SELECT COUNT(*) FROM query_storage"])
        .output();

    let recent_data: Value = match &recent {
        Ok(out) => serde_json::from_slice(&out.stdout).unwrap_or(json!([])),
        Err(_) => json!({"error": "Failed to read recent queries"}),
    };

    let top_data: Value = match &top {
        Ok(out) => serde_json::from_slice(&out.stdout).unwrap_or(json!([])),
        Err(_) => json!({"error": "Failed to read top domains"}),
    };

    let total: i64 = match &stats {
        Ok(out) => {
            let s = String::from_utf8_lossy(&out.stdout);
            s.trim().parse().unwrap_or(0)
        }
        Err(_) => 0,
    };

    (StatusCode::OK, Json(json!({
        "total_queries": total,
        "recent_queries": recent_data,
        "top_domains": top_data,
        "db_path": db_path,
    })))
}

async fn api_routes(Query(_): Query<HashMap<String, String>>) -> (StatusCode, Json<Value>) {
    let output = Command::new("grep")
        .args(["-n", "newServer", "/etc/dnsdist/dnsdist.conf"])
        .output();
    let lines: Vec<String> = match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).lines().map(|l| l.to_string()).collect(),
        Err(_) => vec!["Error reading config".to_string()],
    };
    (StatusCode::OK, Json(json!({"routes": lines})))
}

// ---- Health Monitor ----
async fn run_health_monitor() {
    let mut check_interval = interval(Duration::from_secs(5));
    let mut last_status: HashMap<String, String> = HashMap::new();

    println!("Health monitor started (5s interval)");

    loop {
        check_interval.tick().await;
        let config = load_config("/etc/agentic-dns/config.toml");
        let services = discover_services(&config);

        let mut alerts = vec![];
        let mut failovers = vec![];

        for svc in &services {
            let name = svc["name"].as_str().unwrap_or("");
            let status = svc["status"].as_str().unwrap_or("unknown");
            let prev = last_status.get(name).map(|s| s.as_str()).unwrap_or("unknown");

            if prev == "up" && status != "up" {
                alerts.push(format!("ALERT: {} went DOWN (was {}, now {})", name, prev, status));

                // Auto failover
                if let Some(svc_cfg) = config.services.iter().find(|s| s.name == name) {
                    if let Some(fallback) = &svc_cfg.fallback {
                        failovers.push(format!("FAILOVER: bypassing {} -> using {}", name, fallback));
                    }
                }
            }
            last_status.insert(name.to_string(), status.to_string());
        }

        for alert in &alerts {
            println!("{}", alert);
        }
        for fo in &failovers {
            println!("{}", fo);
            // Extract service and backup names
            if let Some(bypass_idx) = fo.find("bypassing ") {
                if let Some(arrow_idx) = fo.find(" -> ") {
                    let svc_name = fo[bypass_idx + 10..arrow_idx].trim();
                    let backup_name = fo[arrow_idx + 4..].trim();
                    let _ = Command::new("bash")
                        .args(["/usr/local/bin/agentic-dns", "bypass", svc_name, backup_name])
                        .output();
                }
            }
        }

        // Send webhook alerts
        if let Some(webhook) = &config.alert_webhook {
            if !alerts.is_empty() || !failovers.is_empty() {
                let payload = json!({
                    "alerts": alerts,
                    "failovers": failovers,
                    "services": services,
                    "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                });
                let client = reqwest::Client::builder()
                    .timeout(std::time::Duration::from_secs(5))
                    .build()
                    .unwrap();
                let _ = client.post(webhook).json(&payload).send().await;
            }
        }
    }
}

// ---- MCP over stdio ----
fn run_mcp() {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let config = load_config("/etc/agentic-dns/config.toml");

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() {
            continue;
        }
        let parsed: Result<Value, _> = serde_json::from_str(&line);
        if let Ok(req) = parsed {
            let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");
            let fallback_id = json!(1);
            let id = req.get("id").unwrap_or(&fallback_id);

            let response = match method {
                "tools/list" => {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "tools": [
                                {"name": "dns_status", "description": "Show status of all DNS services", "inputSchema": {"type": "object", "properties": {}}},
                                {"name": "dns_query", "description": "Query a domain", "inputSchema": {"type": "object", "properties": {"domain": {"type": "string", "description": "Domain to query"}, "addr": {"type": "string"}, "port": {"type": "string"}}}},
                                {"name": "dns_health", "description": "Check health of all DNS services", "inputSchema": {"type": "object", "properties": {}}},
                                {"name": "dns_routes", "description": "List DNS routing rules", "inputSchema": {"type": "object", "properties": {}}},
                                {"name": "dns_bypass", "description": "Bypass a failing service", "inputSchema": {"type": "object", "properties": {"service": {"type": "string"}, "backup": {"type": "string"}}}},
                            ]
                        }
                    })
                }
                "tools/call" => {
                    let tool_name = req.get("params")
                        .and_then(|p| p.get("name"))
                        .and_then(|n| n.as_str())
                        .unwrap_or("");
                    let args = req.get("params")
                        .and_then(|p| p.get("arguments"))
                        .and_then(|a| a.as_object())
                        .cloned()
                        .unwrap_or_default();

                    let result_text = match tool_name {
                        "dns_status" => {
                            let services = discover_services(&config);
                            let chain = get_chain();
                            format!("Services: {}\nChain: {}", serde_json::to_string_pretty(&services).unwrap_or("?".into()), chain.join("\n-> "))
                        }
                        "dns_query" => {
                            let domain = args.get("domain").and_then(|v| v.as_str()).unwrap_or("example.com");
                            let addr = args.get("addr").and_then(|v| v.as_str()).unwrap_or("127.0.0.1");
                            let port = args.get("port").and_then(|v| v.as_str()).unwrap_or("53");
                            let result = dns_query(domain, addr, port.parse().unwrap_or(53));
                            format!("{} resolved to: {}", domain, result.unwrap_or("no answer".to_string()))
                        }
                        "dns_health" => {
                            let services = discover_services(&config);
                            let results: Vec<String> = services.iter().map(|s| {
                                let name = s["name"].as_str().unwrap_or("?");
                                let addr = s["address"].as_str().unwrap_or("?");
                                let status = s["status"].as_str().unwrap_or("?");
                                format!("{} ({}) [{}]", name, addr, status)
                            }).collect();
                            results.join("\n")
                        }
                        "dns_routes" => {
                            let output = Command::new("grep")
                                .args(["-n", "newServer", "/etc/dnsdist/dnsdist.conf"])
                                .output();
                            match output {
                                Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
                                Err(e) => format!("Error: {}", e),
                            }
                        }
                        "dns_bypass" => {
                            let service = args.get("service").and_then(|v| v.as_str()).unwrap_or("stubby");
                            let backup = args.get("backup").and_then(|v| v.as_str()).unwrap_or("unbound");
                            let output = Command::new("bash")
                                .args(["/usr/local/bin/agentic-dns", "bypass", service, backup])
                                .output();
                            match output {
                                Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
                                Err(e) => format!("Error: {}", e),
                            }
                        }
                        _ => format!("Unknown tool: {}", tool_name),
                    };

                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {"content": [{"text": result_text}]}
                    })
                }
                _ => json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {"content": [format!("Method {} not implemented", method)]}
                }),
            };
            println!("{}", response);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.mcp {
        run_mcp();
        return Ok(());
    }

    if args.monitor {
        run_health_monitor().await;
        return Ok(());
    }

    if args.dot_proxy {
        run_dot_proxy(&args.cert_file, &args.key_file, args.dot_port, &args.upstream_dns).await?;
        return Ok(());
    }

    let app = Router::new()
        .route("/", get(|| async { "agentic-dns API server" }))
        .route("/api/v1/status", get(api_status))
        .route("/api/v1/health", get(api_health))
        .route("/api/v1/query", get(api_query))
        .route("/api/v1/trace", get(api_trace))
        .route("/api/v1/pihole", get(api_pihole))
        .route("/api/v1/routes", get(api_routes));

    use axum::serve;
    let addr = SocketAddr::from(([0, 0, 0, 0], args.api_port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("agentic-dns API server listening on {}", addr);
    println!("Endpoints: /api/v1/status /api/v1/health /api/v1/query /api/v1/trace /api/v1/pihole /api/v1/routes");
    println!("Run with --monitor for health daemon, --mcp for MCP server");

    serve(listener, app).await?;

    Ok(())
}

/// DNS-over-TLS proxy: accepts TLS connections on the specified port,
/// reads DNS queries (2-byte length prefix + wire format), forwards
/// them as plain DNS (UDP) to the upstream resolver (pihole), and
/// returns responses.
async fn run_dot_proxy(cert_file: &str, key_file: &str, dot_port: u16, upstream_dns: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::BufRead as _;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::{TcpListener, UdpSocket};
    use rustls_pemfile::{certs, rsa_private_keys};
    use tokio_rustls::TlsAcceptor;
    use rustls::{ServerConfig, pki_types::{CertificateDer, PrivateKeyDer}};
    use rustls::crypto;

    // Install the aws_lc_rs crypto provider (required by rustls 0.23+)
    let provider = crypto::aws_lc_rs::default_provider();
    provider.install_default().map_err(|e| {
        eprintln!("Warning: crypto provider already installed: {:?}", e);
    }).ok();

    // Load cert and key
    let cert_file_mut = &mut std::fs::File::open(cert_file)?;
    let mut cert_buf = std::io::BufReader::new(cert_file_mut);
    let cert_vec: Vec<CertificateDer<'static>> = certs(&mut cert_buf)
        .filter_map(|c| c.ok())
        .collect();

    let key_file_mut = &mut std::fs::File::open(key_file)?;
    let mut key_buf = std::io::BufReader::new(key_file_mut);
    let key_der: PrivateKeyDer<'static> = rsa_private_keys(&mut key_buf)
        .filter_map(|k| k.ok())
        .next()
        .map(|k| PrivateKeyDer::from(k))
        .ok_or("No RSA private key found (use PKCS#1 format)")?;

    if cert_vec.is_empty() {
        return Err("Failed to load cert".into());
    }

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_vec, key_der)?;

    let acceptor = TlsAcceptor::from(std::sync::Arc::new(config));

    let listener = TcpListener::bind(format!("0.0.0.0:{}", dot_port)).await?;
    println!("DoT proxy listening on 0.0.0.0:{} -> {}", dot_port, upstream_dns);

    let upstream_addr: SocketAddr = format!("{}:53", upstream_dns).parse()?;

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        let acceptor = acceptor.clone();
        let upstream = upstream_addr;

        tokio::spawn(async move {
            let mut tls_stream = match acceptor.accept(stream).await {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("TLS handshake failed from {}: {}", peer_addr, e);
                    return;
                }
            };

            let udp = match UdpSocket::bind("0.0.0.0:0").await {
                Ok(u) => u,
                Err(e) => {
                    eprintln!("Failed to create UDP socket: {}", e);
                    return;
                }
            };

            let mut buf = vec![0u8; 2048];
            loop {
                // Read 2-byte length prefix (DNS-over-TLS framing)
                let mut len_bytes = [0u8; 2];
                match AsyncReadExt::read_exact(&mut tls_stream, &mut len_bytes).await {
                    Ok(2) => {
                        let dns_len = u16::from_be_bytes(len_bytes) as usize;
                        if dns_len == 0 || dns_len > 2048 {
                            break;
                        }
                        if AsyncReadExt::read_exact(&mut tls_stream, &mut buf[..dns_len]).await.is_ok() {
                            // Forward DNS query to upstream via UDP
                            if udp.send_to(&buf[..dns_len], upstream).await.is_ok() {
                                // Read response from upstream
                                let (resp_len, _) = match udp.recv_from(&mut buf).await {
                                    Ok(r) => r,
                                    Err(_) => break,
                                };
                                let resp_len = resp_len as u16;
                                // Send response back to client with 2-byte length prefix
                                let resp_len_bytes = resp_len.to_be_bytes();
                                if AsyncWriteExt::write_all(&mut tls_stream, &resp_len_bytes).await.is_ok() {
                                    let _ = AsyncWriteExt::write_all(&mut tls_stream, &buf[..resp_len as usize]).await;
                                }
                            }
                        }
                    }
                    _ => break,
                }
            }
        });
    }
}
