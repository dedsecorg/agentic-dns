# DNS Chain Services

## Service Table

| Service | Port | Role | Upstream |
|---------|------|------|----------|
| Pi-hole | 53 | Blocking, cache, DHCP | CoreDNS |
| CoreDNS | 5352 | Split-horizon, local zones | dnsdist |
| dnsdist | 5330 | Load balancer, Lua policy | Unbound, DNSCrypt |
| Unbound | 5335 | Recursive resolver | VPN DNS |
| Stubby | 5360 | DoT proxy (TLS) | VPN DNS |
| DNSCrypt | 5354 | DoH proxy | Cloudflare/Quad9 DoH |
| agentic-dns-server | 853 | DoT/mTLS proxy (Rust) | Chain entry |

## Service Definitions (from `bin/agentic-dns`)

```bash
SERVICES=(
    "pihole|53|${PIHOLE_HOST}|forwarder|coredns"
    "coredns|5352|127.0.0.1|split-horizon|dnsdist"
    "dnsdist|5330|127.0.0.1|loadbalancer|unbound,dnscrypt"
    "unbound|5335|127.0.0.1|resolver|vpn-dns"
    "stubby|5360|127.0.0.1|dot-proxy|vpn-dns|unbound"
    "dnscrypt|5354|127.0.0.1|doh-proxy|cloudflare-doh"
    "vpn-dns|53|${VPN_DNS:-unknown}|upstream|none"
)
```

## Health Check Logic

```bash
is_listening() {
    # Checks `ss -tulnp` for port
    # VPN DNS tested via active dig query
}

dns_query() {
    timeout 5 dig +short "$domain" @"$addr" -p "$port"
}

get_service_status() {
    # Returns: up / unhealthy / down
}
```

