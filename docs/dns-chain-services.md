---
layout: default
title: DNS Chain Services
---

# DNS Chain Services

| Service | Port | Role | Upstream |
|---------|------|------|----------|
| Pi-hole | 53 | Blocking, cache, DHCP | CoreDNS |
| CoreDNS | 5352 | Split-horizon, local zones | dnsdist |
| dnsdist | 5330 | Load balancer, Lua policy | Unbound, DNSCrypt |
| Unbound | 5335 | Recursive resolver | VPN DNS |
| Stubby | 5360 | DoT proxy (TLS) | VPN DNS |
| DNSCrypt | 5354 | DoH proxy | Cloudflare/Quad9 DoH |
| agentic-dns-server | 853 | DoT/mTLS proxy (Rust) | Chain entry |

EOF 2>&1
