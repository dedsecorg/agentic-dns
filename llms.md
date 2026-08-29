# agentic-dns

Local agentic DNS stack combining Pi-hole, CoreDNS, dnsdist, and Unbound with an MCP-compatible stdio server for programmatic control.

## Machine-friendly summary

Name: agentic-dns

ShortDescription: Local agentic DNS stack combining Pi-hole, CoreDNS, dnsdist, and Unbound with an MCP-compatible stdio server for programmatic control.

Keywords: agentic-dns, mcp, dns, pi-hole, coredns, dnsdist, unbound, selfhosted, dns-middleware

EntryPoint: ./target/release/agentic-dns

Install:
- git clone https://github.com/dedsecorg/agentic-dns.git
- ./install.sh

CLI Examples:
- Start: `agentic-dns start --config /etc/agentic-dns/config.yml`
- Status: `agentic-dns status`

## MCP Configuration (example)

```yaml
mcp:
  name: agentic-dns
  protocol: stdio
  command: ["./agentic-dns","--mcp"]
  capabilities:
    - dns-query
    - dns-blocklist
    - pcap-monitor
    - nftables-manage
  version: 1.0
```

## Maintainer

- dedsecorg — https://github.com/dedsecorg

## License

MIT
