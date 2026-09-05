---
layout: default
title: Phone DNS Interception
---

# Phone DNS Interception (Android)

```bash
# On server (run as root)
agentic-dns enforce on 100.87.74.22

# On Android: set DNS to server Tailscale IP (100.74.31.18)
# All DoT (port 853) from phone -> Pi-hole -> chain
```

Uses `nftables`/`iptables` TPROXY + `agentic-dns-server` DoT proxy to transparently intercept encrypted DNS (port 853) and force through Pi-hole.

EOF 2>&1
