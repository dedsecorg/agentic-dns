# REST API

## Starting the Server

```bash
agentic-dns api
# Listens on port 8099 (configurable via AGENTIC_DNS_API_PORT)
```

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/v1/status` | Full service status |
| `GET` | `/api/v1/query?domain=example.com` | Resolve through chain |
| `GET` | `/api/v1/trace?domain=example.com` | Trace with packet capture |
| `GET` | `/api/v1/health` | Health checks |
| `GET` | `/api/v1/pihole/log` | Pi-hole logs |
| `POST` | `/api/v1/route` | Add upstream |
| `DELETE` | `/api/v1/route/:name` | Remove upstream |
| `POST` | `/api/v1/bypass` | Failover |

## Examples

```bash
# Full status
curl http://localhost:8099/api/v1/status

# Query
curl http://localhost:8099/api/v1/query?domain=example.com

# Trace
curl http://localhost:8099/api/v1/trace?domain=example.com

# Add upstream
curl -X POST http://localhost:8099/api/v1/route   -H "Content-Type: application/json"   -d '{"name": "backup", "addr": "1.1.1.1:53"}'
```

