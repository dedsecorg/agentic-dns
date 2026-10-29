# MCP Integration

## Starting the Server

```bash
agentic-dns mcp
```

Runs a read-write stdio JSON-RPC 2.0 server. Compatible with Claude Desktop, Cursor, any MCP client.

## Tools Exposed

| Tool | Description | Input |
|------|-------------|-------|
| `dns_status` | All services health + config | `{}` |
| `dns_query` | Resolve domain through chain | `{"domain": "example.com"}` |
| `dns_trace` | Packet capture at each hop | `{"domain": "example.com"}` |
| `dns_route_add` | Add dnsdist upstream | `{"name": "backup", "addr": "1.1.1.1:53"}` |
| `dns_route_remove` | Remove dnsdist upstream | `{"name": "backup"}` |
| `dns_bypass` | Failover service | `{"service": "unbound", "backup": "dnscrypt"}` |
| `dns_enforce` | Phone DNS interception | `{"action": "on", "ip": "100.87.74.22"}` |
| `dns_pihole_log` | Pi-hole query log | `{"limit": 100}` |

## Example Session

```json
// Client -> Server
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}

// Server -> Client
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","capabilities":{"tools":{...}},"serverInfo":{"name":"agentic-dns","version":"1.1.0"}}}

// Client -> Server
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"dns_status","arguments":{}}}
```

## Configuration for Claude Desktop

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

