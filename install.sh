#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 dedsecorg
# agentic-dns One-Command Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/dedsecorg/agentic-dns/main/install.sh | bash

set -euo pipefail

PREFIX="${PREFIX:-/usr/local}"
BIN_DIR="${PREFIX}/bin"
REPO_URL="https://raw.githubusercontent.com/dedsecorg/agentic-dns/main"

log() { echo "[+] $*"; }
warn() { echo "[!] $*"; }

log "Installing agentic-dns to ${BIN_DIR}..."

# Ensure target directory exists
if [ ! -d "$BIN_DIR" ]; then
    sudo mkdir -p "$BIN_DIR"
fi

# Download binary
if command -v curl >/dev/null 2>&1; then
    sudo curl -fsSL "${REPO_URL}/bin/agentic-dns" -o "${BIN_DIR}/agentic-dns"
elif command -v wget >/dev/null 2>&1; then
    sudo wget -qO "${BIN_DIR}/agentic-dns" "${REPO_URL}/bin/agentic-dns"
else
    warn "Neither curl nor wget found. Please install curl or wget."
    exit 1
fi

sudo chmod +x "${BIN_DIR}/agentic-dns"

log "agentic-dns CLI installed successfully at ${BIN_DIR}/agentic-dns"

# Check required Unix tools
MISSING_TOOLS=()
for tool in dig ss tcpdump nft sqlite3 socat jq; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        MISSING_TOOLS+=("$tool")
    fi
done

if [ ${#MISSING_TOOLS[@]} -gt 0 ]; then
    warn "Some recommended diagnostic tools are missing: ${MISSING_TOOLS[*]}"
    warn "Install them via: sudo apt-get install -y bind9-utils iproute2 tcpdump nftables sqlite3 socat jq"
else
    log "All system dependencies (dig, ss, tcpdump, nft, sqlite3, socat, jq) are satisfied!"
fi

echo ""
echo "========================================================"
echo "   agentic-dns Installed Successfully!"
echo "========================================================"
echo ""
echo "Quick Commands:"
echo "  agentic-dns status          - View DNS chain & listening ports"
echo "  agentic-dns trace google.com - Run live inter-service packet trace"
echo "  agentic-dns bypass stubby   - Failover failing resolver to backup"
echo "  agentic-dns mcp             - Run stdio MCP JSON-RPC Server"
echo ""
echo "AI Agent Quick Setup (Claude Code / Cursor / Hermes):"
echo '  Add to .mcp.json:'
echo '  {"mcpServers": {"agentic-dns": {"command": "agentic-dns", "args": ["mcp"]}}}'
echo "========================================================"
