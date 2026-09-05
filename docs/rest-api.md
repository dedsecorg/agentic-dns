---
layout: default
title: REST API
---

# REST API

```bash
agentic-dns api
# or
curl http://localhost:8099/api/v1/status
```

Endpoints:
- `GET /api/v1/status` — full service status
- `GET /api/v1/query?domain=example.com` — resolve
- `GET /api/v1/trace?domain=example.com` — trace
- `GET /api/v1/health` — health checks
- `GET /api/v1/pihole/log` — Pi-hole logs
- `POST /api/v1/route` — add upstream
- `DELETE /api/v1/route/:name` — remove upstream
- `POST /api/v1/bypass` — failover

EOF 2>&1
