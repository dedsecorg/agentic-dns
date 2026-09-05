---
title: Container Deployment
description: Running agentic-dns in containers
---

# Container Deployment

## GHCR Images

Multi-architecture images (linux/amd64, linux/arm64) published to GitHub Container Registry.

```bash
# Pinned release (recommended for CI)
docker pull ghcr.io/dedsecorg/agentic-dns:1.6.0

# Moving major release
docker pull ghcr.io/dedsecorg/agentic-dns:v1

# Latest from main
docker pull ghcr.io/dedsecorg/agentic-dns:latest
```

## Running

### Status Check

```bash
docker run --rm   --cap-add=NET_ADMIN   ghcr.io/dedsecorg/agentic-dns:v1 status
```

### Trace DNS

```bash
docker run --rm   --cap-add=NET_ADMIN   --network=host   ghcr.io/dedsecorg/agentic-dns:v1 trace api.anthropic.com
```

### Run Daemon

```bash
docker run -d   --name agentic-dns   --restart unless-stopped   --cap-add=NET_ADMIN   --network=host   -v /etc/agentic-dns:/etc/agentic-dns   ghcr.io/dedsecorg/agentic-dns:v1 daemon
```

## Volumes

| Host Path | Container Path | Description |
|-----------|----------------|-------------|
| /etc/agentic-dns | /etc/agentic-dns | Service configs |

## Build Locally

```bash
docker build -t agentic-dns .
```

EOF 2>&1
