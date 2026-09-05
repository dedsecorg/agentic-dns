# Troubleshooting

## Services Not Starting

```bash
# Check logs
journalctl -u agentic-dns -n 50 --no-pager
journalctl -u agentic-dns-server -n 50 --no-pager

# Common issues:
# 1. Pi-hole not running
systemctl status pihole-FTL

# 2. Port conflicts
ss -tulnp | grep -E '53|5330|5335|5352|5354|5360|853'

# 3. Missing configs
ls -la /etc/agentic-dns/
```

## DNS Resolution Failing

```bash
# Check each hop
agentic-dns query example.com
agentic-dns trace example.com

# Test individual services
dig @127.0.0.1 -p 53 example.com      # Pi-hole
dig @127.0.0.1 -p 5352 example.com    # CoreDNS
dig @127.0.0.1 -p 5330 example.com    # dnsdist
dig @127.0.0.1 -p 5335 example.com    # Unbound
dig @127.0.0.1 -p 5360 +tls example.com # Stubby
```

## VPN DNS Not Detected

```bash
# Check VPN interfaces
ip link show proton0
ip link show wg0

# Manual override
VPN_DNS=10.2.0.1 agentic-dns status
```

## High CPU/Memory

```bash
# Rust server should use ~2MB RAM
ps aux | grep agentic-dns-server

# Check for runaway loops in bash CLI
top -p $(pgrep -f agentic-dns)
```

## Debug Mode

```bash
# Verbose CLI
AGENTIC_DNS_DEBUG=1 agentic-dns status

# Rust server debug
RUST_LOG=debug agentic-dns-server
```

