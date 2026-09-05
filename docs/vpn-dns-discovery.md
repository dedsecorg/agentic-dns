---
layout: default
title: VPN DNS Discovery
---

# VPN DNS Discovery

Automatically detects VPN DNS endpoints:
- **ProtonVPN** — `protonvpn status` or NetworkManager `proton0` interface
- **NordVPN** — WireGuard config or `nordvpn status`
- **Tailscale** — `tailscale status --json` magic DNS
- **WireGuard** — `wg show` allowed IPs

Set manually: `VPN_DNS=10.2.0.1 agentic-dns status`

EOF 2>&1
