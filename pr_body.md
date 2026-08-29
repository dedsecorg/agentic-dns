This PR adds machine-friendly metadata (llms.txt), a human-readable companion (llms.md), and README edits to improve discoverability by AI crawlers and registries. It includes an MCP configuration snippet, CLI examples, and a keywords section to help indexing and automated retrievers find and interpret agentic-dns.

What I changed:
- Added `llms.txt` at repo root with machine-friendly metadata and MCP snippet.
- Added `llms.md` (human summary) at repo root.
- Updated `README.md` with a Machine-friendly summary, MCP configuration section, CLI examples, and Tags/Keywords.

Recommended follow-ups:
- Add GitHub topics: agentic-dns, mcp, dns, selfhosted, pi-hole (requires repository settings).
- Create a release tag matching the `ReleaseTag` in llms.txt (default v0.1.0) or update llms.txt to match an existing release (e.g., v1.5.0).
- Publish / tag a GHCR Docker image and add the pull badge.
- Post announcement on Hacker News / r/selfhosted and register on MCP Market to build backlinks.
