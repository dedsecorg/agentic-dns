---
layout: default
title: Architecture
---

# Architecture

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

EOF 2>&1
