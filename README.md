# agentic-dns

> POSIX-Native DNS Routing, Diagnostic, and Telemetry Engine for Agentic AI Systems

`agentic-dns` is a zero-dependency, ultra-fast DNS orchestration tool written in pure Bash. Designed for local dev environments, Kubernetes nodes, and AI agents, it manages multi-tier DNS chains (`pihole` -> `coredns` -> `dnsdist` -> `unbound` -> `dnscrypt` -> `VPN DNS`), executes live hop-by-hop packet traces (`trace`), auto-bypasses failing upstreams (`bypass`), and exposes a stdio Model Context Protocol (MCP JSON-RPC) server for AI coding assistants.

---

## Key Features

- **Zero External Dependencies**: Pure POSIX Bash + core Unix utilities (`dig`, `ss`, `tcpdump`, `nft`, `sqlite3`, `socat`, `jq`). Zero memory footprint when idle and sub-10ms CLI response times.
- **Native AI Agent Integration**: Stdio MCP JSON-RPC protocol server out of the box. Connects directly to Claude Code, Cursor, Windsurf, Hermes Agent, and Copilot.
- **Live Inter-Service Tracing (`trace`)**: Runs simultaneous background `tcpdump` captures across loopback and network interfaces to visualize packet hop progression and latency delays.
- **Automated Failover (`bypass`)**: Dynamically rewrites `dnsdist` configuration and reloads routing daemons to bypass failing resolvers in milliseconds.
- **Encrypted DNS Interception (`enforce`)**: Applies non-destructive `nftables` rules to redirect Private DNS (DoT port 853) and drop Google DoQ (UDP/443).
- **REST HTTP API**: Exposes a lightweight local HTTP server on port 8099 for webhooks and integration workflows.

---

## 6-Tier Architecture Pipeline

```
  [Client / Mobile Device]
            |
            v
      pihole (:53)          <- Ad blocking & FTL SQLite telemetry
            |
            v
     coredns (:5352)        <- Split-horizon mesh zone routing (Tailscale / Nebula)
            |
            v
     dnsdist (:5330)        <- Dynamic load balancing & bypass failover policy
            |
   +--------+--------+
   |                 |
   v                 v
unbound (:5335)   dnscrypt-proxy (:5354) / stubby (:5360)
   |                 |
   v                 v
[VPN / Upstream]  [DoH / DoT Cloud Upstreams]
```

---

## Installation

```bash
# Clone the repository
git clone https://github.com/your-username/agentic-dns.git
cd agentic-dns

# Make binary executable and link to PATH
sudo cp bin/agentic-dns /usr/local/bin/agentic-dns
sudo chmod +x /usr/local/bin/agentic-dns
```

---

## CLI Usage

### 1. View Service Status & Active Chain
```bash
agentic-dns status
```
Output:
```
DNS Service Status:
Service            Address                   Type             Status       Listening
-------            -------                   ----             ------       ---------
pihole             127.0.0.1:53              forwarder        up           YES
coredns            127.0.0.1:5352            split-horizon    up           YES
dnsdist            127.0.0.1:5330            loadbalancer     up           YES
unbound            127.0.0.1:5335            resolver         up           YES
stubby             127.0.0.1:5360            dot-proxy        up           YES
dnscrypt           127.0.0.1:5354            doh-proxy        up           YES
```

### 2. Live Hop-by-Hop Packet Trace
```bash
agentic-dns trace google.com
```

### 3. Automatic Failover Bypass
```bash
agentic-dns bypass stubby unbound
```

### 4. Direct Query & Telemetry
```bash
agentic-dns query github.com
agentic-dns pihole-log
agentic-dns health
```

---

## MCP Server Integration for AI Agents

`agentic-dns` includes a stdio MCP server for AI coding assistants (Claude Code, Cursor, Windsurf, Hermes, Copilot).

Add to your `.mcp.json` or agent configuration:

```json
{
  "mcpServers": {
    "agentic-dns": {
      "command": "/usr/local/bin/agentic-dns",
      "args": [
        "mcp"
      ],
      "env": {
        "PIHOLE_HOST": "127.0.0.1",
        "AGENTIC_DNS_API_PORT": "8099"
      }
    }
  }
}
```

Available MCP Tools exposed to AI agents:
- `dns_status` - Query live service status and active chain.
- `dns_query` - Resolve domain through the pipeline.
- `dns_trace` - Execute packet capture trace across inter-service hops.
- `dns_health` - Execute health check sweeps.
- `dns_routes` - List current dnsdist routing rules.
- `dns_pihole_log` - Query SQLite query logs.
- `dns_bypass` - Auto-switch traffic away from broken upstreams.

---

## License

[MIT License](LICENSE) - Free and Open Source.
