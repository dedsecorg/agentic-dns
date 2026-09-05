# Architecture

## DNS Chain Overview

```
                           +------------------+
                           |  Client Query    |
                           +--------+---------+
                                    |
                                    v
                           +------------------+
                           |  Pi-hole (53)    |  -- blocking, cache
                           |  forwarder       |
                           +--------+---------+
                                    |
                                    v
                           +------------------+
                           |  CoreDNS (5352)  |  -- split-horizon, local zones
                           +--------+---------+
                                    |
                                    v
                           +------------------+
                           |  dnsdist (5330)  |  -- load balance, retry, Lua policy
                           +--------+---------+
                    +--------+--------+--------+
                    |                 |        |
                    v                 v        v
             +------------+    +------------+  +------------+
             | Unbound    |    | Stubby     |  | DNSCrypt   |
             | (5335)     |    | (5360)     |  | (5354)     |
             | recursive  |    | DoT proxy  |  | DoH proxy  |
             +-----+------+    +-----+------+  +-----+------+
                   |                 |                |
                   +--------+--------+----------------+
                            |
                            v
                   +------------------+
                   |  VPN DNS         |  -- discovered dynamically
                   |  (Proton/Nord/   |
                   |   Tailscale)     |
                   +------------------+

                           +------------------+
                           |  agentic-dns-    |  -- Rust DoT/mTLS proxy
                           |  server (853)    |     on 127.0.0.1 or Tailscale
                           +------------------+
```

## Data Flow

1. **Client query** hits Pi-hole (port 53)
2. **Pi-hole** blocks/forwards to CoreDNS
3. **CoreDNS** applies split-horizon, forwards to dnsdist
4. **dnsdist** load-balances across Unbound/Stubby/DNSCrypt
5. **Resolvers** query VPN DNS or public DoH
6. **Response** flows back through chain
7. **agentic-dns-server** (Rust) terminates DoT on 853, forwards to Pi-hole

## Event-Driven Reconciliation

Similar to agentic-route:
- `inotifywait` watches `/etc/agentic-dns/` configs
- `agentic-dns` CLI discovers services, health-checks
- VPN DNS changes trigger dnsdist/CoreDNS reconfig
- State emitted to logs/REST API/MCP

