---
layout: default
title: PKI / mTLS operations
---

# PKI / mTLS operations

Every network-reachable control surface in agentic-dns requires a client
certificate:

| Surface | Process | Cert / key / CA (defaults) |
|---------|---------|----------------------------|
| REST API `:8099` | `agentic-dns api` (socat `OPENSSL-LISTEN`) | `AGENTIC_DNS_TLS_CERT` / `AGENTIC_DNS_TLS_KEY` / `AGENTIC_DNS_TLS_CA` → `/etc/agentic-dns/certs/{api.crt,api.key,ca.crt}` |
| DoT proxy `:853` | `agentic-dns-server --dot-proxy` (rustls) | `--cert-file` / `--key-file` / `--client-ca` → `/etc/agentic-dns/certs/{dot.crt,dot.key,ca.crt}` |

The sibling project agentic-route uses the same layout under
`/etc/agentic-route/certs/` (`AGENTIC_ROUTE_TLS_*`). One CA can sign both.

Only *paths* are configurable. Key material lives in `/etc/agentic-*/certs/`
(mode `0700`, root-owned) and is never committed. There is no fallback: if a
file is missing or unreadable the listener refuses to start rather than fall
back to plaintext.

## Threat model

- **Authentication** — X.509 client certs. A peer without a cert chaining to
  `ca.crt` is rejected during the TLS handshake, before any handler code runs
  (socat spawns `api-handler` only after `verify=1` passes; rustls rejects in
  `accept()`). Bash header parsing is never on the auth path.
- **Harvest-now-decrypt-later** — key exchange is hybrid post-quantum
  X25519MLKEM768. The Rust DoT server enforces it (`--allow-classical-kx` to
  also offer X25519 during client migration). The socat surface is pinned to
  TLS 1.3 and inherits whatever groups the host OpenSSL offers: OpenSSL ≥ 3.5
  negotiates X25519MLKEM768 by default; older OpenSSL gives classical
  TLS 1.3 mTLS now and PQ on upgrade, with no config change.
- **Certificates stay classical** (ECDSA P-256 / Ed25519). HNDL is a
  key-exchange problem; signatures only need to be unforgeable *today*, and
  can be rotated onto PQ signature schemes when clients support them.

## Generate the PKI (openssl CLI, no other tooling)

Run as root on the host that owns the services. Use a short-lived CA key
(kept offline if possible) and a per-service server cert.

```bash
umask 077
install -d -m 0700 /etc/agentic-dns/certs /etc/agentic-dns/certs/clients
cd /etc/agentic-dns/certs

# 1. CA (10 years). Keep ca.key offline after issuing.
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:P-256 -nodes \
  -days 3650 -subj "/CN=agentic-ca" -keyout ca.key -out ca.crt

# 2. One server cert per service (1 year). SAN must cover how clients dial it.
for svc in api dot; do
  openssl req -newkey ec -pkeyopt ec_paramgen_curve:P-256 -nodes \
    -subj "/CN=agentic-dns-$svc" \
    -addext "subjectAltName=DNS:$(hostname),DNS:localhost,IP:127.0.0.1" \
    -keyout $svc.key -out $svc.csr
  openssl x509 -req -in $svc.csr -CA ca.crt -CAkey ca.key -CAcreateserial \
    -days 365 -copy_extensions copy -out $svc.crt
  rm -f $svc.csr
done

# 3. One client cert per agent (90 days). Ship <agent>.crt+key to the agent
#    only; the CA never leaves this directory.
agent=hermes
openssl req -newkey ec -pkeyopt ec_paramgen_curve:P-256 -nodes \
  -subj "/CN=agent-$agent" -keyout clients/$agent.key -out clients/$agent.csr
openssl x509 -req -in clients/$agent.csr -CA ca.crt -CAkey ca.key \
  -CAcreateserial -days 90 -out clients/$agent.crt
```

Keys are PKCS#8 PEM (`openssl req -newkey` default); the Rust loader accepts
PKCS#8, SEC1 and PKCS#1.

## Using a client cert

```bash
curl --cacert /etc/agentic-dns/certs/ca.crt \
     --cert ~/.agentic/hermes.crt --key ~/.agentic/hermes.key \
     https://127.0.0.1:8099/api/v1/status

# DoT (kdig / stubby / any DoT client that supports client certs):
kdig +tls +tls-ca=/etc/agentic-dns/certs/ca.crt \
     +tls-keyfile=~/.agentic/hermes.key +tls-certfile=~/.agentic/hermes.crt \
     @127.0.0.1 -p 853 example.com
```

Verify a surface rejects unauthenticated peers:

```bash
openssl s_client -connect 127.0.0.1:853 -CAfile ca.crt </dev/null 2>&1 | grep alert
# expect: sslv3 alert handshake failure  (rustls: "peer sent no certificates")
```

## Rotation

- **Client certs (90 d):** issue a new cert for the agent (step 3), swap on
  the agent, done. Both old and new are valid until expiry; no server restart.
- **Server certs (1 y):** re-run step 2 for the service, then restart it
  (`systemctl restart agentic-dns-api agentic-dns-server`). socat and rustls
  read the files at start-up only.
- **CA (10 y):** create a new CA, issue new server certs, and append the new
  CA to `ca.crt` (it is a bundle — both rustls `RootCertStore` and OpenSSL
  `cafile` accept concatenated PEM) so old and new clients overlap. Re-issue
  client certs, then drop the old CA from the bundle and restart.

## Revocation

There is no CRL/OCSP wiring by design (no new dependencies). Revocation is:

1. **Short lifetimes.** 90-day client certs bound the blast radius.
2. **CA roll.** To revoke one client immediately, roll the CA (above) and
   re-issue every *other* client. With a handful of agents this is minutes.
3. **Emergency:** delete `ca.crt` and restart the service — every peer is
   refused until a CA is restored (fail closed).

Because the REST API and DoT proxy only bind on loopback / Tailscale, a
compromised client cert still requires network reachability to be abused.

## Checklist for a new host

```bash
socat -V | grep -q OPENSSL || echo "socat lacks OpenSSL: API cannot start"
openssl version                              # >= 3.5 for PQ KEM on the socat surface
openssl list -kem-algorithms | grep -qi mlkem && echo "PQ hybrid available to socat"
ls -l /etc/agentic-dns/certs                 # 0700 dir, 0600 keys
agentic-dns api &  agentic-dns-server --dot-proxy &   # both must log "mTLS"
```
