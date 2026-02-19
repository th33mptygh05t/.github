# Claude Code Prompts for th33mptygh05t

## Scaffold a New Rust MCP Server
```
Read AGENTS.md and FRAMEWORK.md. Scaffold a new Rust MCP server called "<name>".
Use the Gitty pattern: single binary, stdio transport, JSON-RPC 2.0, serde, tokio.
Create Cargo.toml, src/main.rs, src/lib.rs, src/tools/ with one initial tool,
Dockerfile (multi-stage, non-root), AGENTS.md, README.md, SECURITY.md.
Commit with: mcp(<name>): initial scaffold
```

## Add a Tool to Existing Rust MCP
```
Read AGENTS.md. Add a new tool called "<name>" to this MCP server.
Create src/tools/<name>.rs, register it in the tool dispatcher,
add tests in tests/, update AGENTS.md. Follow the existing patterns.
```

## Security Audit
```
Read AGENTS.md and SECURITY.md. Audit: hardcoded secrets, pinned deps,
unsafe blocks, input validation, Docker non-root, health endpoints, CVEs.
Report by severity. Fix critical/high. Create issues for medium/low.
```
