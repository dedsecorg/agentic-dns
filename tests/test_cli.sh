#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 dedsecorg
# Test suite for agentic-dns
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="$SCRIPT_DIR/bin/agentic-dns"

echo "=== Running agentic-dns test suite ==="

echo -n "Test 1: Bash syntax check... "
bash -n "$BIN"
echo "PASSED"

echo -n "Test 2: Help flag verification... "
"$BIN" help | grep -q "agentic-dns:"
echo "PASSED"

echo -n "Test 3: Status check compilation... "
"$BIN" status >/dev/null
echo "PASSED"

echo -n "Test 4: MCP tools list JSON-RPC pipe test... "
RESP=$(echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | "$BIN" mcp 2>/dev/null)
echo "$RESP" | grep -q '"dns_status"'
echo "PASSED"

echo -n "Test 5: routes reports all Pi-hole upstreams (single-line + multiline TOML)... "
TOML=$(mktemp)
printf '[dns]\nupstreams = ["127.0.0.1#5330", "127.0.0.1#5335"]\nhosts = ["1.2.3.4 foo"]\ninterface = "eth0"\n' > "$TOML"
PIHOLE_TOML="$TOML" "$BIN" routes | grep -q 'Pi-hole upstreams (pihole.toml): 127.0.0.1#5330,127.0.0.1#5335$'
printf '[dns]\n  upstreams = [\n    "[::1]#5330",\n    "10.0.0.2#53",\n  ]\n  hosts = ["9.9.9.9 bar"]\n' > "$TOML"
PIHOLE_TOML="$TOML" "$BIN" routes | grep -q 'Pi-hole upstreams (pihole.toml): \[::1\]#5330,10.0.0.2#53$'
rm -f "$TOML"
PIHOLE_TOML="$TOML" "$BIN" routes | grep -q 'Pi-hole upstreams' && { echo "FAILED (stale upstreams after file removal)"; exit 1; }
echo "PASSED"

echo "=== All tests passed cleanly ==="
