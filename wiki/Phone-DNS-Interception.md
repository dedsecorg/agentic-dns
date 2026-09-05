# Phone DNS Interception (Android)

## Setup

```bash
# On server (run as root)
agentic-dns enforce on 100.87.74.22

# On Android: set DNS to server Tailscale IP (100.74.31.18)
# All DoT (port 853) from phone -> Pi-hole -> chain
```

## How It Works

1. **nftables/iptables TPROXY** intercepts outbound port 853 (DoT) from phone IP
2. **agentic-dns-server** (Rust) terminates TLS on port 853
3. **Decrypted query** forwarded to local Pi-hole (port 53)
4. **Response** encrypted and sent back to phone

## Supported Protocols

| Protocol | Port | Status |
|----------|------|--------|
| DoT (DNS over TLS) | 853 | **Implemented** |
| DoH (DNS over HTTPS) | 443 | Not implemented |
| DoQ (DNS over QUIC) | 784 | Not implemented |

Only port 853 DoT is verified and documented. Remove 443/784 claims unless implemented.

