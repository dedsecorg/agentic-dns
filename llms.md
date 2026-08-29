# agentic-dns

Local agentic DNS stack combining Pi-hole, CoreDNS, dnsdist, and Unbound with an MCP-compatible stdio server for programmatic control.

## Machine-friendly summary

Name: agentic-dns

ShortDescription: Local agentic DNS stack combining Pi-hole, CoreDNS, dnsdist, and Unbound with an MCP-compatible stdio server for programmatic control.

Keywords: agentic-dns, mcp, dns, pi-hole, coredns, dnsdist, unbound, selfhosted, dns-middleware

EntryPoint: bin/agentic-dns

Install:
- git clone https://github.com/dedsecorg/agentic-dns.git
- ./install.sh

CLI Examples:
- MCP server (stdio JSON-RPC): `agentic-dns mcp`
- REST API server: `agentic-dns api`

## MCP Configuration (example)

```yaml
mcp:
  name: agentic-dns
  protocol: stdio
  command: ["agentic-dns","mcp"]
  capabilities:
    - dns_status
    - dns_query
    - dns_trace
    - dns_health
    - dns_routes
    - dns_pihole_log
    - dns_bypass
  version: 1.0
```

## Maintainer

- dedsecorg — https://github.com/dedsecorg

## License

MIT
