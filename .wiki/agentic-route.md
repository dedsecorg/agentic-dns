# Agentic Route

**Declarative kernel policy-routing reconciler.** Define desired routing state in JSON; the daemon continuously reconciles the Linux kernel routing tables and policy rules to match — event-driven, idempotent, and hardened for production.

## Quick Links
- [GitHub Repository](https://github.com/dedsecorg/agentic-route)
- [README](https://github.com/dedsecorg/agentic-route#readme)

## Integration with agentic-dns

| Layer | Tool | Purpose |
|-------|------|---------|
| L3 (Routing) | agentic-route | Kernel policy routing, VPN traffic steering |
| L7 (DNS) | agentic-dns | DNS chain orchestration, DoT/DoH proxy |

Both use the same architectural patterns:
- Declarative intent files
- Event-driven daemons (inotify + Netlink)
- FIFO multiplexing with debounce
- Read-only MCP servers
- REST APIs

## Shared Infrastructure

```
+------------------+     +------------------+
|  agentic-route   |     |  agentic-dns     |
|  (L3 routing)    |     |  (L7 DNS)        |
+--------+---------+     +--------+---------+
         |                       |
         |  Tailscale mesh       |  Pi-hole blocking
         |  NordVPN egress       |  CoreDNS split-horizon
         |  ProtonVPN DNS        |  dnsdist load balance
         v                       v
+----------------------------------------------+
|           Linux Kernel                       |
|  ip rule/route + nftables/iptables           |
+----------------------------------------------+
```

