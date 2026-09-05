---
layout: default
title: Installation
---

# Installation

## Manual

```bash
git clone https://github.com/dedsecorg/agentic-dns
cd agentic-dns
sudo -S -p '' ./install.sh
```

Installs:
- `/usr/local/bin/agentic-dns` — main CLI
- `/usr/local/bin/agentic-dns-server` — Rust DoT/mTLS proxy
- `/etc/agentic-dns/` — configs for dnsdist, CoreDNS, Unbound, Stubby, DNSCrypt
- `/etc/systemd/system/agentic-dns.service` — systemd unit
- `/etc/systemd/system/agentic-dns-server.service` — Rust server unit

## Smithery (for MCP clients)

```bash
npx -y @smithery/cli install @dedsecorg/agentic-dns
```

## Requirements

- Linux (systemd, nftables/iptables)
- Pi-hole, CoreDNS, dnsdist, Unbound, Stubby, DNSCrypt-Proxy installed
- `dig`, `ss`, `jq`, `bash` 4.4+
- Rust 1.75+ (for server build)
- `CAP_NET_ADMIN`, `CAP_NET_BIND_SERVICE` (for server)

EOF 2>&1
