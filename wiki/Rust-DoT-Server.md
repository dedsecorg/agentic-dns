# Rust DoT/mTLS Server

## Overview

`agentic-dns-server` — standalone binary:
- Terminates TLS on 853 (DoT) or 443 (DoH)
- Validates client certs (mTLS optional)
- Forwards to local DNS chain (Pi-hole -> chain)
- Metrics on `:9090/metrics` (Prometheus)
- Zero-copy, async, ~2MB RAM

## Build

```bash
cd server && cargo build --release
# Binary at target/release/agentic-dns-server
# Or download prebuilt from releases
```

## Systemd Unit

```ini
[Unit]
Description=agentic-dns DoT/mTLS proxy server
After=network-online.target

[Service]
Type=simple
ExecStart=/usr/local/bin/agentic-dns-server
Restart=always
CapabilityBoundingSet=CAP_NET_BIND_SERVICE
AmbientCapabilities=CAP_NET_BIND_SERVICE
NoNewPrivileges=yes

[Install]
WantedBy=multi-user.target
```

## Configuration

| Env Var | Default | Description |
|---------|---------|-------------|
| `DOT_PORT` | 853 | DoT listen port |
| `DOH_PORT` | 443 | DoH listen port |
| `METRICS_PORT` | 9090 | Prometheus metrics |
| `UPSTREAM_DNS` | 127.0.0.1:53 | Upstream resolver (Pi-hole) |
| `TLS_CERT` | auto-generated | TLS certificate path |
| `TLS_KEY` | auto-generated | TLS key path |
| `CLIENT_CA` | optional | mTLS client CA |

