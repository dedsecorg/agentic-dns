---
title: systemd Service
description: Running agentic-dns as a systemd service
---

# systemd Service

## Installation

The `install.sh` script creates systemd units:

```bash
sudo -S -p '' ./install.sh
```

## Service Files

### agentic-dns.service

```ini
[Unit]
Description=agentic-dns daemon
After=network.target
Wants=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/agentic-dns daemon
Restart=on-failure
RestartSec=5
CapabilityBoundingSet=CAP_NET_ADMIN CAP_NET_BIND_SERVICE
AmbientCapabilities=CAP_NET_ADMIN CAP_NET_BIND_SERVICE
NoNewPrivileges=true

[Install]
WantedBy=multi-user.target
```

### agentic-dns-server.service

```ini
[Unit]
Description=agentic-dns DoT/mTLS proxy server
After=network.target
Wants=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/agentic-dns-server
Restart=on-failure
RestartSec=5
CapabilityBoundingSet=CAP_NET_BIND_SERVICE
AmbientCapabilities=CAP_NET_BIND_SERVICE
NoNewPrivileges=true
User=agentic-dns

[Install]
WantedBy=multi-user.target
```

## Enable and Start

```bash
sudo -S -p '' systemctl enable agentic-dns agentic-dns-server
sudo -S -p '' systemctl start agentic-dns agentic-dns-server
sudo -S -p '' systemctl status agentic-dns agentic-dns-server
```

## Logs

```bash
journalctl -u agentic-dns -f
journalctl -u agentic-dns-server -f
```

EOF 2>&1
