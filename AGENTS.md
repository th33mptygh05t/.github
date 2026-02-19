# AGENTS.md — th33mptygh05t Org Standards

## Overview
Org-wide standards. All projects inherit these rules.

## Core Principles
- Don't reinvent the wheel — use existing tools, wrap and extend them
- Rust for everything unless there's a strong reason not to
- Simple > fancy — working MVP first, polish later
- Every repo has AGENTS.md — no exceptions

## Code Style
- Rust: clippy clean, rustfmt enforced, edition 2021+
- TypeScript (frontend only): strict mode, single quotes, no semicolons
- Commits: `type(scope): description` — types: feat, fix, sec, docs, refactor, test, chore, mcp

## Security — Non-Negotiable
- Never commit .env, keys, tokens, or credentials
- Pin all dependencies to exact versions
- Validate all external input
- Docker containers run as non-root
- Destructive operations require confirmation

## Testing
- All new features include tests
- Run `cargo test` / `npm test` before pushing
- Minimum 80% coverage target

## Deployment
- All projects target k3s on Contabo HQ (6 CPU, 12GB RAM, 200GB disk)
- Use Helm charts for deployment
- Set resource limits on every container
- Health check endpoints required

## MCP Conventions (Rust)
- One concern per MCP server
- Tool names are verbs: get_pods, scan_threats, check_cost
- JSON-RPC 2.0 over stdio
- Serde for serialization, strong typing for all tool params
- Destructive tools clearly documented
- Single binary, logs to stderr only

## File Structure
- Every repo: AGENTS.md, README.md, SECURITY.md, LICENSE
- Rust: src/, tests/, Cargo.toml, Dockerfile, helm/
- Frontend: src/, tests/, package.json, Dockerfile, helm/

## Git Workflow
- Branches: main ← dev ← feat/*, fix/*, sec/*
- Security patches fast-track to main
- PRs require passing tests
- No auto-merge on security changes
