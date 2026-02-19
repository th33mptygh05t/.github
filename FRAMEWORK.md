# FRAMEWORK.md — th33mptygh05t Engineering Framework

## Philosophy

- **One engineer, infinite agents.** Every project is built to be operated by AI agents, with one human setting direction.
- **Agents are employees.** Every repo has an AGENTS.md that tells agents how to work in that repo. No exceptions.
- **MCPs are products.** Internal automation gets extracted as MCP servers. If it's useful internally, it ships as a tool.
- **Don't reinvent the wheel.** Wrap, extend, and orchestrate existing tools. Never build what already exists and works.
- **Rust for everything.** Career signal + performance + safety. All agents, MCPs, CLIs, and backends are Rust.
- **Simple > fancy.** Working MVP beats polished vaporware. No unnecessary abstractions. Ship first, refine later.
- **Security is the foundation, not a feature.** Baked in from day one, not bolted on after.
- **70/30 AI split.** Humans do core logic and architecture. Agents do scaffolding, boilerplate, and repetitive work.

## The Stack

### Languages
| Language | Use Case |
|----------|----------|
| Rust | Agents, MCPs, CLIs, backends — everything |
| Next.js + TypeScript | Frontends only (ghostdeck) |
| Bash | Glue scripts, deployment helpers |

### Key Rust Crates
| Crate | Purpose |
|-------|---------|
| tokio | Async runtime |
| serde / serde_json | Serialization |
| reqwest | HTTP client |
| clap | CLI argument parsing |
| git2 | Git operations (libgit2 bindings) |
| octocrab | GitHub API client |
| sqlx | Database (PostgreSQL) |

### Infrastructure
| Component | Role |
|-----------|------|
| k3s | Lightweight Kubernetes |
| Helm | Package management / deployment |
| Docker | Containerization |
| PostgreSQL | Primary database |
| Redis | Caching / pub-sub |
| iptables | Firewall |
| Traefik | Ingress / reverse proxy |

### AI
| Model | Role |
|-------|------|
| Claude Opus | Orchestrator — complex reasoning, architecture |
| Claude Sonnet | Code generation — implementation, refactoring |
| Claude Haiku | Verification — reviews, checks, quick tasks |
| Claude Code | Local development — IDE-integrated agent |

## Rust Project Structure Standard

```
project-name/
├── AGENTS.md
├── README.md
├── SECURITY.md
├── LICENSE
├── Cargo.toml
├── Dockerfile
├── helm/
│   ├── Chart.yaml
│   ├── values.yaml
│   └── templates/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── tools/          # MCP tools (one file per tool)
│   ├── config.rs
│   └── error.rs
├── tests/
└── docs/
```

## Rust MCP Server Pattern (based on Gitty)

- **Single binary** — one `main.rs` entry point, compiles to a single executable
- **stdio transport** — reads JSON-RPC from stdin, writes to stdout
- **JSON-RPC 2.0** — standard protocol for all tool communication
- **Logs to stderr only** — stdout is reserved for MCP protocol messages
- **git2 for local reads, shell out for writes/remote** — safe local operations via libgit2, complex/remote operations via git CLI
- **Token resolution chain:** config file → environment variable → CLI tool fallback (e.g. `gh auth token`)
- **Config location:** `~/.config/<project>/config.toml`
- **Registration:** add to Claude Code at `~/.claude.json` under `mcpServers`

### Tool Implementation Pattern
```rust
// src/tools/example_tool.rs
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ExampleInput {
    pub param: String,
}

#[derive(Serialize)]
pub struct ExampleOutput {
    pub result: String,
}

pub fn execute(input: ExampleInput) -> Result<ExampleOutput, Box<dyn std::error::Error>> {
    // Implementation here
    Ok(ExampleOutput { result: "done".to_string() })
}
```

## Deployment Standard

### Target
All projects deploy to **k3s on Contabo HQ**.

### Resource Budget
| Resource | Total | Reserved (system) | Available |
|----------|-------|-------------------|-----------|
| CPU | 6 cores | 1 core | 5 cores |
| RAM | 12 GB | 2 GB | 10 GB |
| Disk | 200 GB | 20 GB | 180 GB |

### Per-Service Defaults
| Resource | Request | Limit |
|----------|---------|-------|
| CPU | 100m | 500m |
| RAM | 64Mi | 256Mi |

### Deployment Flow
1. `git push` to main
2. SSH to Contabo HQ
3. `helm upgrade --install <service> ./helm -f helm/values.yaml`

### Deployment Checklist
- [ ] Docker image builds successfully
- [ ] Helm values configured (image, tag, replicas, resources)
- [ ] Resource limits set on every container
- [ ] Health check endpoint responds (`/healthz` or tool-based)
- [ ] Secrets stored in k3s secrets (not in values.yaml)
- [ ] Ingress configured (if externally accessible)
- [ ] iptables rules updated (if new ports needed)

## Security Standard

### Requirements
- Pin all dependencies to exact versions in Cargo.toml / package.json
- Scan for CVEs regularly (`cargo audit`, `npm audit`)
- Encrypt all secrets at rest and in transit
- Log all security-relevant events
- No root containers — ever
- Egress firewall — containers only reach what they need
- Self-monitor — agents watch each other

### Incident Response
1. **Isolate** — take affected service offline
2. **Capture** — snapshot logs, state, evidence
3. **Rebuild** — redeploy from known-good state
4. **Post-mortem** — document what happened and why
5. **Update framework** — add prevention to this document

## Git Workflow

### Branch Strategy
```
main ← dev ← feat/*, fix/*, sec/*
```

- `main` — production, always deployable
- `dev` — integration branch, PRs merge here first
- `feat/*` — new features
- `fix/*` — bug fixes
- `sec/*` — security patches (fast-track to main)

### Commit Convention
```
type(scope): description
```

| Type | Use |
|------|-----|
| feat | New feature |
| fix | Bug fix |
| sec | Security patch |
| docs | Documentation |
| refactor | Code restructure |
| test | Tests |
| chore | Maintenance |
| mcp | MCP server changes |

## Org Map

```
th33mptygh05t/
├── .github/              # Framework, standards
├── gitty/                # 🦀 Git/GitHub MCP (SHIPPED ✅)
├── cloudmind/            # ☁️ AWS management (Rust)
├── forgemaster/          # 🔧 Provisioning (Rust)
├── spectre-mail/         # 📧 Email agent (Rust)
├── spectre-whatsapp/     # 💬 WhatsApp agent (Rust)
├── spectre-slack/        # 🔔 Slack agent (Rust)
├── ghostdeck/            # 🖥️ Dashboard (Next.js + Rust API)
├── panam/                # 🧠 Orchestrator + organizer (Rust)
└── bulwark/              # 🛡️ Security platform (future)
```
