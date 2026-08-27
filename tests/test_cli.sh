#!/bin/bash
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

echo "=== All tests passed cleanly ==="
