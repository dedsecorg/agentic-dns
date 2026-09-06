---
layout: default
title: Installation
---

# Installation

## Quick Install (One-Line)

```bash
curl -fsSL https://raw.githubusercontent.com/dedsecorg/agentic-dns/main/install.sh | bash
```

Downloads the prebuilt binary from the latest release, installs to `/usr/local/bin/agentic-dns`, verifies system dependencies.

## Manual Install from Release Assets

```bash
# Download and verify
curl -fsSL -O https://github.com/dedsecorg/agentic-dns/releases/download/v1.6.0/agentic-dns-v1.6.0.tar.gz
curl -fsSL -O https://github.com/dedsecorg/agentic-dns/releases/download/v1.6.0/checksums.txt
sha256sum -c checksums.txt

# Extract and install
tar -xzf agentic-dns-v1.6.0.tar.gz
sudo install -m 755 agentic-dns /usr/local/bin/agentic-dns
```

Each release includes:
- `agentic-dns-v<version>.tar.gz` — prebuilt `agentic-dns` CLI binary
- `checksums.txt` — SHA256 checksums for verification

## Docker / GHCR (Multi-Arch)

```bash
# Pull latest
docker pull ghcr.io/dedsecorg/agentic-dns:latest

# Or pinned version
docker pull ghcr.io/dedsecorg/agentic-dns:v1.6.0

# Run
docker run --rm -it \
  --cap-add=NET_ADMIN --cap-add=NET_BIND_SERVICE \
  --network host \
  ghcr.io/dedsecorg/agentic-dns:latest agentic-dns status
```

Multi-arch images: `linux/amd64`, `linux/arm64`.

## Full System Install (Server)

```bash
git clone https://github.com/dedsecorg/agentic-dns
cd agentic-dns
sudo -S -p '' ./install.sh
```

Installs:
- `/usr/local/bin/agentic-dns` — main CLI
- `/usr/local/bin/agentic-dns-server` — Rust DoT/mTLS proxy
- `/etc/agentic-dns/` — configs for dnsdist, CoreDNS, Unbound, Stubby, DNSCrypt
- `/etc/systemd/system/agentic-dns.service` — systemd unit
- `/etc/systemd/system/agentic-dns-server.service` — Rust server unit

Enable services:

```bash
sudo systemctl enable --now agentic-dns
sudo systemctl enable --now agentic-dns-server
```

## Build from Source

### CLI (Bash)
```bash
git clone https://github.com/dedsecorg/agentic-dns
cd agentic-dns
sudo ./install.sh
```

### Rust DoT/mTLS Server
```bash
cd agentic-dns
cargo build --release
sudo install -m 755 target/release/agentic-dns-server /usr/local/bin/agentic-dns-server
```

Requires Rust 1.75+.

## Smithery (for MCP Clients)

```bash
npx -y @smithery/cli install @dedsecorg/agentic-dns
```

Adds `agentic-dns` as an MCP server to your client config.

## Requirements

- Linux (systemd, nftables/iptables)
- Pi-hole, CoreDNS, dnsdist, Unbound, Stubby, DNSCrypt-Proxy installed
- `dig`, `ss`, `jq`, `bash` 4.4+
- Rust 1.75+ (for server build)
- `CAP_NET_ADMIN`, `CAP_NET_BIND_SERVICE` (for server)

## Verify Installation

```bash
agentic-dns status
agentic-dns trace google.com
agentic-dns --version
```