# th33mptygh05t Rust MCP Server Template

Based on patterns proven in Gitty, Raikage, Yamato, and Ino.

## Usage

1. Copy this directory: `cp -r templates/mcp-server ~/th33mptygh05t/<new-agent>`
2. Find and replace `{{PROJECT_NAME}}` with your agent name
3. Find and replace `{{PROJECT_DESCRIPTION}}` with a one-liner
4. Add your tools in `src/tools/`
5. Register in `src/tools/registry.rs` tool dispatcher
6. Update AGENTS.md with tool documentation

## What's Included

- MCP server with stdio transport, JSON-RPC 2.0
- Config loading from `~/.config/{{PROJECT_NAME}}/config.toml`
- Error handling via thiserror
- Logging via tracing (stderr only)
- Health check tool
- Dockerfile (multi-stage, non-root)
- AGENTS.md, SECURITY.md, LICENSE

## Register in Claude Code

Add to `~/.claude.json`:
```json
{
  "mcpServers": {
    "{{PROJECT_NAME}}": {
      "command": "/home/kerolos/.local/bin/{{PROJECT_NAME}}",
      "args": ["--stdio"]
    }
  }
}
```

## Stack

- Rust 2021 edition
- tokio (async runtime)
- serde + serde_json (serialization)
- clap (CLI parsing)
- tracing (logging to stderr)
- thiserror (error types)
- toml (config)
- reqwest (HTTP, if needed)
