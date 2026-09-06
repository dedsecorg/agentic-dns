---
layout: default
title: REST API
---

# REST API

The API is mTLS-only (TLS 1.3, client cert required, verified against
`AGENTIC_DNS_TLS_CA`). See [PKI / mTLS operations](pki.md).

```bash
agentic-dns api
# or
curl --cacert /etc/agentic-dns/certs/ca.crt --cert agent.crt --key agent.key \
     https://localhost:8099/api/v1/status
```

Endpoints (all read-only; responses are `{"status":"ok","text":"..."}`):
- `GET /api/v1/status` — full service status
- `GET /api/v1/health` — health checks
- `GET /api/v1/routes` — dnsdist upstreams
- `GET /api/v1/query?domain=example.com` — resolve
- `GET /api/v1/trace?domain=example.com` — trace
- `GET /api/v1/pihole/log` — Pi-hole logs

Mutations (`route add/remove`, `bypass`, `enforce`) are CLI/MCP-only and are
deliberately not exposed over the network.

EOF 2>&1
