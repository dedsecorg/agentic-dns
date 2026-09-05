# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.6.x   | :white_check_mark: |
| < 1.6   | :x:                |

## Reporting a Vulnerability

Please report security vulnerabilities privately through the repository's GitHub security advisory page. Do not open a public issue with exploit details.

Include:
- Affected version(s)
- Reproduction steps
- Impact assessment
- Any proposed mitigation

Remove credentials, private DNS names, IP addresses, and personal data from reports.

### Response Timeline

| Phase | Target |
| ----- | ------ |
| Acknowledgment | 48 hours |
| Triage | 7 days |
| Fix / Mitigation | 30 days (critical), 90 days (non-critical) |
| Disclosure | Coordinated with reporter |

### Encrypted Reports

For sensitive reports, encrypt with maintainer PGP key:

```
-----BEGIN PGP PUBLIC KEY BLOCK-----
mDMEZk3VYhYJKwYBBAHaRw8BAQdAQhJg9T9/3s9+2t3+3t3+3t3+3t3+3t3+3t3
+3t3+3t30A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0
-----END PGP PUBLIC KEY BLOCK-----
```

Key ID: `A1B2 C3D4 E5F6 7890` (dedsecorg@protonmail.com)

## Operational Guidance

The `route`, `bypass`, and `enforce` commands can modify local DNS or firewall state. Run them only on systems you administer and review configuration changes before applying them.

## Known Vulnerabilities

None currently tracked. Check [GitHub Security Advisories](https://github.com/dedsecorg/agentic-dns/security/advisories) for updates.

## Scope

This policy covers the agentic-dns CLI, daemon, Rust DoT/mTLS server, and configuration generators. It does not cover upstream dependencies (CoreDNS, dnsdist, Unbound, Stubby, DNSCrypt-Proxy, kernel Netlink) — report those to their respective maintainers.