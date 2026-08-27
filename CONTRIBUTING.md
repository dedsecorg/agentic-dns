# Contributing

## Before submitting a change

1. Open an issue for substantial behavior or interface changes.
2. Keep changes focused and preserve the zero-dependency design.
3. Do not include credentials, private DNS data, or machine-specific configuration.

## Local validation

Run:

```bash
bash -n bin/agentic-dns
./tests/test_cli.sh
npm pack --dry-run
```

Pull requests should explain the user impact, include relevant tests, and document changes to DNS routing, firewall, Docker, or package behavior.
