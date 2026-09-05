---
title: Smithery Installation
description: Install agentic-dns via Smithery for MCP clients
---

# Smithery Installation

## Quick Install

```bash
npx -y @smithery/cli install @dedsecorg/agentic-dns
```

This configures the MCP server for your AI client (Claude Desktop, Cursor, etc.).

## Manual Configuration

Add to your MCP client config:

```json
{
  "mcpServers": {
    "agentic-dns": {
      "command": "npx",
      "args": ["-y", "@smithery/cli", "run", "@dedsecorg/agentic-dns"],
      "env": {}
    }
  }
}
```

## Available Tools

When installed via Smithery, the following MCP tools are available:

- `dns_status` — All services health + config
- `dns_query` — Resolve domain through chain
- `dns_trace` — Packet capture at each hop
- `dns_route_add` — Add dnsdist upstream
- `dns_route_remove` — Remove dnsdist upstream
- `dns_bypass` — Failover service
- `dns_enforce` — Phone DNS interception
- `dns_pihole_log` — Pi-hole query log

EOF 2>&1
