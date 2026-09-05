---
layout: default
title: MCP Integration
---

# MCP Integration

Start read-write MCP server:
```bash
agentic-dns mcp
```

Tools exposed:
- `dns_status` — all services health + config
- `dns_query` — resolve domain through chain
- `dns_trace` — packet capture at each hop
- `dns_route_add` — add dnsdist upstream
- `dns_route_remove` — remove dnsdist upstream
- `dns_bypass` — failover service
- `dns_enforce` — phone DNS interception
- `dns_pihole_log` — Pi-hole query log

EOF 2>&1
