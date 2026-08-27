# agentic-dns - LLM Knowledge Base & Integration Reference

> POSIX-Native DNS Routing, Diagnostic, and Telemetry Engine for Agentic AI Systems.

## Project Overview

`agentic-dns` is a zero-dependency, ultra-fast DNS orchestration engine written in pure Bash and Rust. It manages multi-tier DNS chains (`pihole` -> `coredns` -> `dnsdist` -> `unbound` -> `dnscrypt` -> `VPN DNS`), executes live hop-by-hop packet traces (`trace`), auto-bypasses failing upstreams (`bypass`), and exposes a stdio Model Context Protocol (MCP JSON-RPC) server for AI coding assistants.

---

## 1-Line Installation

```bash
curl -fsSL https://raw.githubusercontent.com/dedsecorg/agentic-dns/main/install.sh | bash
```

---

## CLI Usage Reference

- `agentic-dns status`: Show live service status, IP addresses, listening ports, and active DNS chain.
- `agentic-dns query <domain>`: Resolve domain through the primary DNS entrypoint.
- `agentic-dns trace <domain>`: Execute live simultaneous background packet captures across loopback and network interfaces to visualize packet hop progression.
- `agentic-dns routes`: List active routing rules from `dnsdist` configuration.
- `agentic-dns route add <name> <addr:port>`: Add upstream resolver server to `dnsdist`.
- `agentic-dns route remove <name>`: Remove upstream server from `dnsdist`.
- `agentic-dns bypass <failing_service> [backup_service]`: Dynamically bypass a failing DNS resolver and reroute traffic to a healthy backup.
- `agentic-dns enforce <on|off|status|clean> [target_ip]`: Apply non-destructive `nftables` rules to intercept Private DNS (DoT port 853) and drop Google DoQ (UDP/443).
- `agentic-dns pihole-log`: Query recent DNS queries and top clients from Pi-hole SQLite database (`/etc/pihole/pihole-FTL.db`).
- `agentic-dns health`: Execute health check sweeps across all registered DNS listeners.
- `agentic-dns tag`: Show service classification tags (listening, entry-point, upstream, fallback).
- `agentic-dns api`: Start local REST HTTP API server on port 8099.
- `agentic-dns mcp`: Start stdio MCP JSON-RPC protocol server for AI agents.

---

## Model Context Protocol (MCP) JSON-RPC Tools

Exposed MCP Tools:
- `dns_status`: Retrieve status of all services and active chain.
- `dns_query`: Resolve domain through the pipeline.
- `dns_trace`: Spawns live packet capture trace across inter-service hops.
- `dns_health`: Runs health check sweep across listeners.
- `dns_routes`: Lists current dnsdist backend rules.
- `dns_pihole_log`: Queries SQLite query log.
- `dns_bypass`: Auto-switches routing around failing upstreams.

---

## AI Agent Integration Configuration

Add to `.mcp.json`:
```json
{
  "mcpServers": {
    "agentic-dns": {
      "command": "agentic-dns",
      "args": ["mcp"],
      "env": {
        "PIHOLE_HOST": "127.0.0.1",
        "AGENTIC_DNS_API_PORT": "8099"
      }
    }
  }
}
```
