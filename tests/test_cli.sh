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

mcp_call() { # $1 = tool name, $2 = arguments JSON; prints the tool's text result
    printf '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"%s","arguments":%s}}\n' "$1" "${2:-{\}}" \
        | "$BIN" mcp 2>/dev/null | jq -r '.result.content[0].text'
}

# Shims: sudo passes through, nft/systemctl operate on files in $SANDBOX.
SANDBOX=$(mktemp -d)
trap 'rm -rf "$SANDBOX"' EXIT
mkdir -p "$SANDBOX/bin" "$SANDBOX/state"
cat > "$SANDBOX/bin/sudo" <<'SH'
#!/usr/bin/env bash
exec "$@"
SH
cat > "$SANDBOX/bin/nft" <<'SH'
#!/usr/bin/env bash
case "$*" in
    "list ruleset") cat "$SANDBOX/nft.state" ;;
    "-f -") grep -v '^flush ruleset$' > "$SANDBOX/nft.state" ;;
    *) exit 1 ;;
esac
SH
cat > "$SANDBOX/bin/systemctl" <<'SH'
#!/usr/bin/env bash
sleep "${SLOW_SYSTEMCTL:-0}"
echo "$*" >> "$SANDBOX/systemctl.log"
SH
cat > "$SANDBOX/bin/nslookup" <<'SH'
#!/usr/bin/env bash
cat >/dev/null
echo "nslookup-shim-done"
SH
chmod +x "$SANDBOX/bin/"*
export SANDBOX PATH="$SANDBOX/bin:$PATH" AGENTIC_DNS_STATE_DIR="$SANDBOX/state" \
    AGENTIC_DNS_DNSDIST_CONF="$SANDBOX/dnsdist.conf" AGENTIC_DNS_DEADMAN_SECONDS=3
reset_sandbox() {
    echo "table ip nat { }" > "$SANDBOX/nft.state"
    echo "-- original" > "$SANDBOX/dnsdist.conf"
    rm -f "$SANDBOX/systemctl.log"
}

echo -n "Test 6: sys_exec allowlist, no shell, stdin isolated from JSON-RPC... "
mcp_call sys_exec '{"command":"rm -rf /"}' | grep -qx 'Error: Command not allowed'
mcp_call sys_exec '{"command":"ip route add default via 1.2.3.4"}' | grep -qx 'Error: Command arguments not allowed'
# shellcheck disable=SC2016
mcp_call sys_exec '{"command":"dig $(id)"}' | grep -qx 'Error: Command arguments not allowed'
# A stdin-reading command must not swallow the next request on the same session.
RESP=$(printf '%s\n' \
    '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"sys_exec","arguments":{"command":"nslookup"}}}' \
    '{"jsonrpc":"2.0","id":2,"method":"ping"}' | "$BIN" mcp 2>/dev/null)
echo "$RESP" | grep -q 'nslookup-shim-done'
echo "$RESP" | jq -e 'select(.id == 2)' >/dev/null
echo "PASSED"

echo -n "Test 7: deadman start returns immediately; timeout restores nftables + dnsdist... "
reset_sandbox
T0=$(date +%s)
mcp_call sys_deadman_start | grep -q 'Deadman switch armed'
[ $(( $(date +%s) - T0 )) -lt "$AGENTIC_DNS_DEADMAN_SECONDS" ] || { echo "FAILED (start blocked until timeout)"; exit 1; }
mcp_call sys_deadman_start | grep -q 'already running'
echo "table ip nat { chain PREROUTING { dnat to 10.0.0.1:853 } }" > "$SANDBOX/nft.state"
echo "-- changed" > "$SANDBOX/dnsdist.conf"
sleep $(( AGENTIC_DNS_DEADMAN_SECONDS + 2 ))
grep -qx 'table ip nat { }' "$SANDBOX/nft.state"
grep -qx -- '-- original' "$SANDBOX/dnsdist.conf"
grep -qx 'restart dnsdist' "$SANDBOX/systemctl.log"
grep -q 'rollback complete' "$SANDBOX/state/deadman.log"
[ -z "$(find "$SANDBOX/state" -maxdepth 1 -name 'backup.*')" ]
echo "PASSED"

echo -n "Test 8: deadman commit keeps changes... "
reset_sandbox
mcp_call sys_deadman_start | grep -q 'Deadman switch armed'
echo "-- changed" > "$SANDBOX/dnsdist.conf"
mcp_call sys_deadman_commit | grep -q 'disarmed'
sleep $(( AGENTIC_DNS_DEADMAN_SECONDS + 2 ))
grep -qx -- '-- changed' "$SANDBOX/dnsdist.conf"
[ ! -e "$SANDBOX/systemctl.log" ]
mcp_call sys_deadman_commit | grep -q 'No deadman switch active'
echo "PASSED"

echo -n "Test 9: deadman refuses to arm without a complete backup... "
reset_sandbox
rm -f "$SANDBOX/dnsdist.conf"
mcp_call sys_deadman_start | grep -q 'switch NOT armed'
[ ! -e "$SANDBOX/state/deadman.pid" ] || ! kill -0 "$(cat "$SANDBOX/state/deadman.pid")" 2>/dev/null
[ -z "$(find "$SANDBOX/state" -maxdepth 1 -name 'backup.*')" ]
echo "PASSED"

echo -n "Test 10: no re-arm or commit while a rollback is in progress... "
reset_sandbox
export SLOW_SYSTEMCTL=3 # inherited by the timer, so its rollback takes ~3s
mcp_call sys_deadman_start | grep -q 'Deadman switch armed'
echo "-- changed" > "$SANDBOX/dnsdist.conf"
sleep $(( AGENTIC_DNS_DEADMAN_SECONDS + 1 ))
[ -e "$SANDBOX/state/rolling_back" ]
mcp_call sys_deadman_start | grep -q 'already running'
mcp_call sys_deadman_commit | grep -q 'changes NOT committed'
sleep 4
unset SLOW_SYSTEMCTL
grep -qx -- '-- original' "$SANDBOX/dnsdist.conf"
[ ! -e "$SANDBOX/state/rolling_back" ] && [ ! -e "$SANDBOX/state/deadman.pid" ]
echo "PASSED"

echo "=== All tests passed cleanly ==="
