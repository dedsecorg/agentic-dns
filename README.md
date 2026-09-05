# agentic-dns

**Agentic AI-Native DNS Routing, Diagnostics & Telemetry.** Manages a complete DNS resolution chain (Pi-hole -> CoreDNS -> dnsdist -> Unbound/Stubby/DNSCrypt -> VPN DNS) with CLI, REST API, stdio MCP, and a Rust DoT/mTLS proxy server. Pure POSIX Bash + standard Unix tools -- zero Python/Node dependencies.

[![Release](https://img.shields.io/github/v/release/dedsecorg/agentic-dns?color=blue&logo=github)](https://github.com/dedsecorg/agentic-dns/releases)
[![GHCR Image](https://img.shields.io/badge/ghcr.io-dedsecorg%2Fagentic--dns-24292e?logo=docker)](https://github.com/dedsecorg/agentic-dns/pkgs/container/agentic-dns)
[![Multi-Arch](https://img.shields.io/badge/arch-amd64%20%7C%20arm64-blue)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Context7](https://img.shields.io/badge/Context7-Indexed-brightgreen)](https://context7.com/dedsecorg/agentic-dns)

---

### OCI Container (GHCR)

Pre-built multi-architecture images (`linux/amd64`, `linux/arm64`) for ephemeral agent sandboxes and headless runners.

#### Pulling the Image

```bash
# Pinned release
docker pull ghcr.io/dedsecorg/agentic-dns:1.6.0

# Moving major release
docker pull ghcr.io/dedsecorg/agentic-dns:v1

# Latest tracking branch
docker pull ghcr.io/dedsecorg/agentic-dns:latest
```

#### Running the Resolver Health & Trace Tools

```bash
# Quick health check
docker run --rm \
  --cap-add=NET_ADMIN \
  ghcr.io/dedsecorg/agentic-dns:v1 status

# Run trace check against host resolver
docker run --rm \
  --cap-add=NET_ADMIN \
  --network=host \
  ghcr.io/dedsecorg/agentic-dns:v1 trace api.anthropic.com
```

---

### Context7 Documentation

This repository is indexed on [Context7](https://context7.com/dedsecorg/agentic-dns) with 134 code snippets for AI-assisted development. Use the Context7 MCP server or visit the link above for searchable documentation.

```bash
# Query via Context7 MCP
# "How to install agentic-dns?"
# "agentic-dns commands reference"
# "DNS chain architecture"
```

---

## Why This Exists

Modern networks run multiple DNS layers simultaneously: Pi-hole for blocking, CoreDNS for split-horizon, dnsdist for load balancing, Unbound for recursive resolution, Stubby/DNSCrypt for DoT/DoH, plus VPN-provided DNS. When any layer fails or VPNs rotate DNS, resolution breaks silently.

**agentic-dns** treats the DNS chain as **managed infrastructure**:

1. **Discovers topology** -- detects running services, VPN DNS endpoints, listening ports
2. **Health-checks each hop** -- active DNS queries against every resolver
3. **Auto-reconfigures** -- rewrites dnsdist/CoreDNS upstreams when VPN DNS changes
4. **Exposes telemetry** -- REST API, MCP server, structured logs for AI agents
5. **Provides DoT/mTLS proxy** -- Rust server terminates TLS, forwards to chain

---

## Architecture

```text
                           +------------------+
                           |  Client Query    |
                           +--------+---------+
                                    |
                                    v
                           +------------------+
                           |  Pi-hole (53)    |  -- blocking, cache
                           |  forwarder       |
                           +--------+---------+
                                    |
                                    v
                           +------------------+
                           |  CoreDNS (5352)  |  -- split-horizon, local zones
                           +--------+---------+
                                    |
                                    v
                           +------------------+
                           |  dnsdist (5330)  |  -- load balance, retry, Lua policy
                           +--------+---------+
                    +--------+--------+--------+
                    |                 |        |
                    v                 v        v
             +------------+    +------------+  +------------+
             | Unbound    |    | Stubby     |  | DNSCrypt   |
             | (5335)     |    | (5360)     |  | (5354)     |
             | recursive  |    | DoT proxy  |  | DoH proxy  |
             +-----+------+    +-----+------+  +-----+------+
                   |                 |                |
                   +--------+--------+----------------+
                            |
                            v
                   +------------------+
                   |  VPN DNS         |  -- discovered dynamically
                   |  (Proton/Nord/   |
                   |   Tailscale)     |
                   +------------------+

                           +------------------+
                           |  agentic-dns-    |  -- Rust DoT/mTLS proxy
                           |  server (853)    |     on 127.0.0.1 or Tailscale
                           +------------------+
```

---

## Installation

### Manual
```bash
git clone https://github.com/dedsecorg/agentic-dns
cd agentic-dns
sudo -S -p '' -S -p '' ./install.sh
```

Installs:
- `/usr/local/bin/agentic-dns` -- main CLI
- `/usr/local/bin/agentic-dns-server` -- Rust DoT/mTLS proxy (prebuilt or cargo build)
- `/etc/agentic-dns/` -- configs for dnsdist, CoreDNS, Unbound, Stubby, DNSCrypt
- `/etc/systemd/system/agentic-dns.service` -- systemd unit
- `/etc/systemd/system/agentic-dns-server.service` -- Rust server unit

### Smithery (for MCP clients)
```bash
npx -y @smithery/cli install @dedsecorg/agentic-dns
```

---

## Commands

| Command | Description |
|---------|-------------|
| `agentic-dns status` | Show all DNS services, addresses, types, health |
| `agentic-dns query <domain>` | Resolve through full chain |
| `agentic-dns trace <domain>` | Trace DNS path with live packet capture at each hop |
| `agentic-dns routes` | List current dnsdist routing rules |
| `agentic-dns route add <name> <addr:port>` | Add upstream to dnsdist |
| `agentic-dns route remove <name>` | Remove upstream from dnsdist |
| `agentic-dns bypass <service> [backup]` | Bypass failing service (auto-switch to backup) |
| `agentic-dns enforce {on,off,status,clean} [ip]` | Intercept phone encrypted DNS -> Pi-hole |
| `agentic-dns pihole-log` | Show recent Pi-hole DNS query log |
| `agentic-dns health` | Check health of all DNS services |
| `agentic-dns tag` | Show service tags |
| `agentic-dns api` | Start REST API server on port 8099 |
| `agentic-dns mcp` | Start stdio MCP JSON-RPC server |
| `agentic-dns server` | Start Rust DoT/mTLS proxy server |

---

## DNS Chain Services

| Service | Port | Role | Upstream |
|---------|------|------|----------|
| Pi-hole | 53 | Blocking, cache, DHCP | CoreDNS |
| CoreDNS | 5352 | Split-horizon, local zones | dnsdist |
| dnsdist | 5330 | Load balancer, Lua policy | Unbound, DNSCrypt |
| Unbound | 5335 | Recursive resolver | VPN DNS |
| Stubby | 5360 | DoT proxy (TLS) | VPN DNS |
| DNSCrypt | 5354 | DoH proxy | Cloudflare/Quad9 DoH |
| agentic-dns-server | 853 | DoT/mTLS proxy (Rust) | Chain entry |

---

## VPN DNS Discovery

Automatically detects VPN DNS endpoints:
- **ProtonVPN** -- `protonvpn status` or NetworkManager `proton0` interface
- **NordVPN** -- WireGuard config or `nordvpn status`
- **Tailscale** -- `tailscale status --json` magic DNS
- **WireGuard** -- `wg show` allowed IPs

Set manually: `VPN_DNS=10.2.0.1 agentic-dns status`

---

## Configuration

### `/etc/agentic-dns/` -- Service Configs

Generated by install.sh, editable for custom zones/upstreams:
- `dnsdist.conf` -- routing rules, load balancing, Lua policies
- `Corefile` -- CoreDNS zones, plugins
- `unbound.conf` -- recursive resolver settings
- `stubby.yml` -- DoT upstream certificates
- `dnscrypt-proxy.toml` -- DoH server stamps

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PIHOLE_HOST` | 127.0.0.1 | Pi-hole API address |
| `PIHOLE_API_KEY` | /etc/pihole/pihole-api-key.txt | Pi-hole API key |
| `AGENTIC_DNS_API_PORT` | 8099 | REST API port |
| `VPN_DNS` | auto | Override VPN DNS |
| `AGENTIC_DNS_SERVER_BIN` | agentic-dns-server | Rust server binary |

---

## MCP Integration

Start read-write MCP server:
```bash
agentic-dns mcp
```

Tools exposed:
- `dns_status` -- all services health + config
- `dns_query` -- resolve domain through chain
- `dns_trace` -- packet capture at each hop
- `dns_route_add` -- add dnsdist upstream
- `dns_route_remove` -- remove dnsdist upstream
- `dns_bypass` -- failover service
- `dns_enforce` -- phone DNS interception
- `dns_pihole_log` -- Pi-hole query log

---

## REST API

```bash
agentic-dns api
# or
curl http://localhost:8099/api/v1/status
```

Endpoints:
- `GET /api/v1/status` -- full service status
- `GET /api/v1/query?domain=example.com` -- resolve
- `GET /api/v1/trace?domain=example.com` -- trace
- `GET /api/v1/health` -- health checks
- `GET /api/v1/pihole/log` -- Pi-hole logs
- `POST /api/v1/route` -- add upstream
- `DELETE /api/v1/route/:name` -- remove upstream
- `POST /api/v1/bypass` -- failover

---

## Phone DNS Interception (Android)

```bash
# On server (run as root)
agentic-dns enforce on 100.87.74.22

# On Android: set DNS to server Tailscale IP (100.74.31.18)
# All DoT (port 853) from phone -> Pi-hole -> chain
```

Uses `nftables`/`iptables` TPROXY + `agentic-dns-server` DoT proxy to transparently intercept encrypted DNS (port 853) and force through Pi-hole.

---

## Rust DoT/mTLS Server

`agentic-dns-server` -- standalone binary:
- Terminates TLS on 853 (DoT) or 443 (DoH)
- Validates client certs (mTLS optional)
- Forwards to local DNS chain
- Metrics on `:9090/metrics` (Prometheus)
- Zero-copy, async, ~2MB RAM

Build:
```bash
cd server && cargo build --release
# or download prebuilt from releases
```

---

## Requirements

- Linux (systemd, nftables/iptables)
- Pi-hole, CoreDNS, dnsdist, Unbound, Stubby, DNSCrypt-Proxy installed
- `dig`, `ss`, `jq`, `bash` 4.4+
- Rust 1.75+ (for server build)
- `CAP_NET_ADMIN`, `CAP_NET_BIND_SERVICE` (for server)

---

## License

MIT -- see [LICENSE](LICENSE).

---

## Related

- **agentic-route** -- kernel policy routing reconciler (same author)
- **hermes-dns** -- private fork with real IPs, same engine