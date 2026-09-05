# Security Hardening

## Systemd Unit Hardening (`agentic-dns-server.service`)

```ini
[Service]
Type=simple
ExecStart=/usr/local/bin/agentic-dns-server
Restart=always
CapabilityBoundingSet=CAP_NET_BIND_SERVICE
AmbientCapabilities=CAP_NET_BIND_SERVICE
NoNewPrivileges=yes
ProtectSystem=strict
ReadWritePaths=/etc/agentic-dns /run/agentic-dns
```

| Setting | Purpose |
|---------|---------|
| `CAP_NET_BIND_SERVICE` | Bind to privileged ports (853, 443) |
| `NoNewPrivileges=yes` | Prevents privilege escalation |
| `ProtectSystem=strict` | Read-only `/usr`, `/boot`, `/etc` |
| `ReadWritePaths` | Only config/state dirs writable |

## Main CLI Hardening (`agentic-dns.service`)

```ini
CapabilityBoundingSet=CAP_NET_ADMIN CAP_NET_BIND_SERVICE
AmbientCapabilities=CAP_NET_ADMIN CAP_NET_BIND_SERVICE
ProtectSystem=strict
ReadWritePaths=/etc/agentic-dns /run/agentic-dns /etc/pihole
```

## Attack Surface

| Vector | Mitigation |
|--------|------------|
| Malicious configs | Only root writes `/etc/agentic-dns/` |
| TLS cert injection | Auto-generated or user-provided, root-only |
| Phone IP spoofing | TPROXY rules match specific phone IP |
| DNS rebinding | Pi-hole blocks local/private zones |

## Audit Commands

```bash
# Verify capabilities
getcap /usr/local/bin/agentic-dns-server
getcap /usr/local/bin/agentic-dns

# Verify systemd hardening
systemctl show agentic-dns-server --property=CapabilityBoundingSet,NoNewPrivileges,ProtectSystem
systemctl show agentic-dns --property=CapabilityBoundingSet,ProtectSystem

# Verify file permissions
ls -la /usr/local/bin/agentic-dns* /etc/agentic-dns/
```

