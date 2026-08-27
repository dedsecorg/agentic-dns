# Model Context Protocol (MCP) Integration Guide

`agentic-dns` includes a native stdio Model Context Protocol server (`agentic-dns mcp`) allowing AI coding assistants to inspect, query, and manage DNS infrastructure programmatically.

## Supported Clients

- **Claude Code**: `claude mcp add agentic-dns agentic-dns mcp`
- **Cursor**: Add to `.cursor/mcp.json`
- **Windsurf**: Add to `.windsurf/mcp.json`
- **Hermes Agent**: Managed via `mcp_server.json`
- **GitHub Copilot**: Configurable in Copilot workspace settings

## Protocol Specification

- **Transport**: stdio (standard input/output JSON-RPC 2.0)
- **Tool Listing**: `tools/list`
- **Tool Execution**: `tools/call`

## Available Tools

- `dns_status`: Retrieve status of all services and active chain.
- `dns_query`: Test domain resolution through the pipeline.
- `dns_trace`: Spawns live tcpdump packet capture on inter-service hops.
- `dns_health`: Runs health check sweep across listeners.
- `dns_routes`: Lists current dnsdist backend rules.
- `dns_pihole_log`: Queries SQLite query log.
- `dns_bypass`: Auto-switches routing around failing upstreams.
