# agentic-dns

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI Status](https://github.com/dedsecorg/agentic-dns/workflows/CI/badge.svg)](https://github.com/dedsecorg/agentic-dns/actions)
[![Protocol: MCP](https://img.shields.io/badge/MCP-JSON--RPC-success.svg)](docs/MCP_GUIDE.md)
[![Docker: GHCR](https://img.shields.io/badge/Docker-GHCR-blue.svg)](https://github.com/dedsecorg/agentic-dns/pkgs/container/agentic-dns)
[![GitHub Stars](https://img.shields.io/github/stars/dedsecorg/agentic-dns?style=flat&color=yellow)](https://github.com/dedsecorg/agentic-dns/stargazers)
[![Release](https://img.shields.io/badge/Release-v1.5.0-blue)](https://github.com/dedsecorg/agentic-dns/releases)

> POSIX-Native DNS Routing, Diagnostic, and Telemetry Engine for Agentic AI Systems (Claude, Cursor, Windsurf, Hermes Agent, Copilot).

`agentic-dns` is a zero-dependency, ultra-fast DNS orchestration engine written in pure Bash and Rust. Designed for developer workstations, homelabs, Kubernetes nodes, and autonomous AI agents, it manages multi-tier DNS chains (`pihole` -> `coredns` -> `dnsdist` -> `unbound` -> `dnscrypt` -> `VPN DNS`), executes live hop-by-hop packet traces (`trace`), auto-bypasses failing upstreams (`bypass`), and exposes a stdio Model Context Protocol (MCP JSON-RPC) server for AI coding assistants.

---

## Quick Installation Options

### Option 1: 1-Line Script (Linux / macOS)
```bash
curl -fsSL https://raw.githubusercontent.com/dedsecorg/agentic-dns/main/install.sh | bash
```

### Option 2: Docker Container (GHCR)
```bash
docker run -d --net=host ghcr.io/dedsecorg/agentic-dns:latest
```

### Option 3: Node.js / npx Package
```bash
npx agentic-dns status
```

---

## 60-Second AI Agent Onboarding

`agentic-dns` provides out-of-the-box MCP JSON-RPC protocol support for AI agents.

### 1. Claude Code
```bash
claude mcp add agentic-dns agentic-dns mcp
```

### 2. Cursor / Windsurf / Copilot
Add to your project or user `.mcp.json` file:
```json
{
  "mcpServers": {
    "agentic-dns": {
      "command": "agentic-dns",
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

### 3. Hermes Agent
Run `hermes mcp add agentic-dns agentic-dns mcp`.

---

## Example AI Assistant Prompts

Once connected, ask your AI coding assistant:

- *"Check if any local DNS resolvers in the chain are down or unhealthy."*
- *"Trace the packet path for github.com across loopback and outbound interfaces."*
- *"Bypass stubby and route DNS traffic through unbound."*
- *"Show recent Pi-hole query statistics from SQLite."*

---

## Key Features

- **Zero External Dependencies**: Core POSIX Bash execution + standard Unix tools (`dig`, `ss`, `tcpdump`, `nft`, `sqlite3`, `socat`, `jq`). Zero idle RAM usage.
- **Native Model Context Protocol (MCP)**: Exposes `dns_status`, `dns_query`, `dns_trace`, `dns_health`, `dns_routes`, `dns_pihole_log`, and `dns_bypass` to AI agents.
- **Live Inter-Service Tracing (`trace`)**: Runs background `tcpdump` captures across loopback and network interfaces to visualize packet hop progression and latency bottlenecks.
- **Automated Failover (`bypass`)**: Dynamically rewrites `dnsdist` configuration and reloads routing daemons to bypass failing resolvers in milliseconds.
- **Encrypted DNS Interception (`enforce`)**: Applies non-destructive `nftables` rules to redirect Private DNS (DoT port 853) and drop Google DoQ (UDP/443).
- **High-Performance Rust Server (`agentic-dns-server`)**: Includes `server/` crate for DoT/DoH proxy handling and REST telemetry.

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

## CLI Command Quick Reference

```bash
# View active DNS chain status and listening ports
agentic-dns status

# Execute live inter-service packet trace
agentic-dns trace google.com

# Failover failing service to backup
agentic-dns bypass stubby unbound

# Check health of all DNS listeners
agentic-dns health

# View Pi-hole query logs
agentic-dns pihole-log

# Start REST API server on port 8099
agentic-dns api

# Start stdio MCP JSON-RPC Server
agentic-dns mcp
```

---

## License

[MIT License](LICENSE) - Free and Open Source.
