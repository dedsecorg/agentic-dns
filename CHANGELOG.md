# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- MCP deadman switch (`sys_deadman_start` / `sys_deadman_commit`): snapshots the nftables ruleset and dnsdist config, and restores both if not committed within 60 seconds. Refuses to arm if the snapshot fails.
- MCP `sys_exec`: allowlisted read-only diagnostics (`ip`/`ss`/`ping`/`traceroute`/`dig`/`nslookup`/`resolvectl`/`systemctl`/`journalctl`), no shell, 30s timeout.
- CI/CD workflow enhancements, including ShellCheck.
- Pull request template for standardized contributions.
