# Installation

## Manual (Recommended for Servers)

```bash
git clone https://github.com/dedsecorg/agentic-dns
cd agentic-dns
sudo -S -p '' -S -p '' ./install.sh
```

### What Gets Installed

| Path | Purpose |
|------|---------|
| `/usr/local/bin/agentic-dns` | Main CLI |
| `/usr/local/bin/agentic-dns-server` | Rust DoT/mTLS proxy (prebuilt or cargo build) |
| `/etc/agentic-dns/` | Configs for dnsdist, CoreDNS, Unbound, Stubby, DNSCrypt |
| `/etc/systemd/system/agentic-dns.service` | Main systemd unit |
| `/etc/systemd/system/agentic-dns-server.service` | Rust server unit |

### Enable Services

```bash
sudo -S -p '' systemctl enable --now agentic-dns
sudo -S -p '' systemctl enable --now agentic-dns-server
```

## Smithery (For MCP Clients)

```bash
npx -y @smithery/cli install @dedsecorg/agentic-dns
```

## Requirements

- Linux (systemd, nftables/iptables)
- Pi-hole, CoreDNS, dnsdist, Unbound, Stubby, DNSCrypt-Proxy installed
- `dig`, `ss`, `jq`, `bash` 4.4+
- Rust 1.75+ (for server build)
- `CAP_NET_ADMIN`, `CAP_NET_BIND_SERVICE` (for server)

