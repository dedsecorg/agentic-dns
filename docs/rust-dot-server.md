---
layout: default
title: Rust DoT Server
---

# Rust DoT/mTLS Server

`agentic-dns-server` — standalone binary:
- Terminates TLS on 853 (DoT) or 443 (DoH)
- Validates client certs (mTLS optional)
- Forwards to local DNS chain
- Metrics on `:9090/metrics` (Prometheus)
- Zero-copy, async, ~2MB RAM

Build:
```bash
cd server && cargo build --release
# or download prebuilt from releases
```

EOF 2>&1
