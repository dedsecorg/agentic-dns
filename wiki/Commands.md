# Commands

## Overview

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

## Status Output

```bash
$ agentic-dns status
agentic-dns status
Pi-hole (53): up
CoreDNS (5352): up
dnsdist (5330): up
Unbound (5335): up
Stubby (5360): up
DNSCrypt (5354): up
VPN DNS (10.2.0.1): up
```

## Health Checks

Each service checked via active DNS query:
- `up` — listening and responding
- `unhealthy` — listening but query failed
- `down` — not listening

