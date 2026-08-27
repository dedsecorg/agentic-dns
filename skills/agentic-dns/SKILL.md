---
name: agentic-dns
description: Agentic AI-native multi-tier DNS routing, live packet tracing, and failover bypass engine for Pi-hole, CoreDNS, dnsdist, Unbound, Stubby, DNSCrypt-Proxy, and VPN DNS.
---

# agentic-dns Skill

`agentic-dns` provides a zero-dependency POSIX Bash and Rust engine for inspecting, diagnosing, routing, and failing over multi-tier DNS infrastructure.

## CLI Commands

| Command | Syntax | Description |
|---------|--------|-------------|
| `status` | `agentic-dns status` | Show status of all DNS services, addresses, listening ports, and active chain. |
| `query` | `agentic-dns query <domain>` | Resolve domain through entrypoint DNS. |
| `trace` | `agentic-dns trace <domain>` | Execute simultaneous tcpdump packet capture on loopback and network interfaces. |
| `routes` | `agentic-dns routes` | Show active rules in dnsdist config. |
| `route add` | `agentic-dns route add <name> <addr:port>` | Add new upstream server to dnsdist. |
| `route remove` | `agentic-dns route remove <name>` | Remove upstream server from dnsdist. |
| `bypass` | `agentic-dns bypass <failing> [backup]` | Bypass a failing resolver and prioritize backup. |
| `enforce` | `agentic-dns enforce <on\|off\|status\|clean> [target_ip]` | Intercept encrypted DNS (DoT port 853 -> pihole, DROP DoQ UDP/443). |
| `pihole-log` | `agentic-dns pihole-log` | Query recent DNS log and top clients from Pi-hole SQLite database. |
| `health` | `agentic-dns health` | Run health check sweep across listeners. |
| `tag` | `agentic-dns tag` | Show service classification tags. |
| `api` | `agentic-dns api` | Start REST API server on port 8099. |
| `mcp` | `agentic-dns mcp` | Start stdio MCP JSON-RPC Server for AI agents. |

## Standard Workflows

### 1. Diagnose DNS Connectivity Outage
```bash
agentic-dns status
agentic-dns health
agentic-dns trace google.com
```

### 2. Bypass Failing Upstream
```bash
agentic-dns bypass stubby unbound
```

### 3. MCP JSON-RPC Configuration for Agents
```json
{
  "mcpServers": {
    "agentic-dns": {
      "command": "agentic-dns",
      "args": ["mcp"]
    }
  }
}
```
