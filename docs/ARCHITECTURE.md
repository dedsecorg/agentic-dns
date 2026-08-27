# agentic-dns Architecture Specification

## Overview

`agentic-dns` manages a multi-tier DNS resolution pipeline designed for local development, mesh networks (Tailscale / Nebula), ad-blocking, and secure upstream forwarding (DoH / DoT / VPN DNS).

## Service Layer Breakdown

1. **pihole (:53)**
   - Entrypoint forwarder. Handles ad-blocking, domain filtering, and queries stored in `/etc/pihole/pihole-FTL.db`.
2. **coredns (:5352)**
   - Split-horizon DNS resolver for internal mesh domain zones (`.mesh`, `.internal`, `.home`).
3. **dnsdist (:5330)**
   - High-performance load balancer and failover policy router.
4. **unbound (:5335)**
   - Recursive DNS resolver with local DNSSEC validation.
5. **dnscrypt-proxy (:5354) / stubby (:5360)**
   - Encrypted DNS proxies handling DNS-over-HTTPS (DoH) and DNS-over-TLS (DoT).

## Extensible Metadata Array

Services are declared in a simple pipe-delimited format:
```bash
SERVICES=(
    "pihole|53|127.0.0.1|forwarder|coredns"
    "coredns|5352|127.0.0.1|split-horizon|dnsdist"
    "dnsdist|5330|127.0.0.1|loadbalancer|unbound,dnscrypt"
    "unbound|5335|127.0.0.1|resolver|vpn-dns"
    "stubby|5360|127.0.0.1|dot-proxy|vpn-dns|unbound"
    "dnscrypt|5354|127.0.0.1|doh-proxy|cloudflare-doh"
    "vpn-dns|53|${VPN_DNS:-unknown}|upstream|none"
)
```
Adding a new DNS daemon requires appending a single metadata line to this array.
