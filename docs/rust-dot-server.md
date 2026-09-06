---
layout: default
title: Rust DoT Server
---

# Rust DoT/mTLS Server

`agentic-dns-server` — standalone binary:
- Terminates TLS on 853 (DoT) or 443 (DoH)
- Requires client certs chaining to `--client-ca` (mTLS, mandatory; see [PKI](pki.md))
- Hybrid post-quantum key exchange only (X25519MLKEM768 via aws-lc-rs; `--allow-classical-kx` to also offer X25519)
- Forwards to local DNS chain
- Metrics on `:9090/metrics` (Prometheus)
- Zero-copy, async, ~2MB RAM

Build:
```bash
cd server && cargo build --release
# or download prebuilt from releases
```

Run:
```bash
agentic-dns-server --dot-proxy \
  --cert-file /etc/agentic-dns/certs/dot.crt \
  --key-file  /etc/agentic-dns/certs/dot.key \
  --client-ca /etc/agentic-dns/certs/ca.crt \
  --upstream-dns 127.0.0.1 --dot-port 853
```

Keys may be PKCS#8, SEC1 or PKCS#1 PEM.

Other modes (all read `--config`, default `/etc/agentic-dns/config.toml`, once at
startup):

- no flag — mTLS REST API on `--api-port` (8099), same `/api/v1/*` routes as
  `agentic-dns api` and the same `--cert-file`/`--key-file`/`--client-ca`
  requirement. There is no plaintext listener.
- `--monitor` — health daemon polling every `health_check_interval_secs`.
- `--mcp` — stdio MCP server.

