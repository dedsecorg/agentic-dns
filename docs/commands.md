---
layout: default
title: Commands
---

# Commands

| Command | Description |
|---------|-------------|
| `agentic-dns status` | Show all DNS services, addresses, types, health |
| `agentic-dns query <domain>` | Resolve through full chain |
| `agentic-dns trace <domain>` | Trace DNS path with live packet capture |
| `agentic-dns routes` | List current dnsdist routing rules |
| `agentic-dns route add <name> <addr:port>` | Add upstream to dnsdist |
| `agentic-dns route remove <name>` | Remove upstream from dnsdist |
| `agentic-dns bypass <service> [backup]` | Bypass failing service |
| `agentic-dns enforce {on,off,status,clean} [ip]` | Intercept phone encrypted DNS |
| `agentic-dns pihole-log` | Show recent Pi-hole DNS query log |
| `agentic-dns health` | Check health of all DNS services |
| `agentic-dns tag` | Show service tags |
| `agentic-dns api` | Start REST API server on port 8099 |
| `agentic-dns mcp` | Start stdio MCP JSON-RPC server |
| `agentic-dns server` | Start Rust DoT/mTLS proxy server |

EOF 2>&1
