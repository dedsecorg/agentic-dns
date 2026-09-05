# VPN DNS Discovery

## Automatic Detection

```bash
discover_vpn_dns() {
    # 1. NetworkManager proton0 interface
    nmcli dev show proton0 | grep "IP4.DNS"

    # 2. WireGuard allowed IPs
    wg show wg0 | grep "allowed ips"

    # 3. protonvpn CLI
    protonvpn status | grep -oP 'DNS:\s*\K[0-9.]+'
}
```

## Supported VPNs

| VPN | Detection Method |
|-----|------------------|
| ProtonVPN | `protonvpn status` / NetworkManager `proton0` |
| NordVPN | WireGuard config / `nordvpn status` |
| Tailscale | `tailscale status --json` (Magic DNS) |
| WireGuard | `wg show` allowed IPs |

## Manual Override

```bash
VPN_DNS=10.2.0.1 agentic-dns status
VPN_DNS=10.2.0.1 agentic-dns query example.com
```

