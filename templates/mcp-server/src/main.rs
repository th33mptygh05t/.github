mod config;
mod error;
mod mcp;
mod tools;

use clap::Parser;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

use crate::config::AppConfig;
use crate::mcp::server::McpServer;
use crate::tools::registry::ToolRegistry;

#[derive(Parser)]
#[command(
    name = "{{PROJECT_NAME}}",
    version,
    about = "{{PROJECT_DESCRIPTION}}"
)]
struct Cli {
    #[arg(long)]
    stdio: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .json()
        .init();

    if !cli.stdio {
        eprintln!("{{PROJECT_NAME}}: {{PROJECT_DESCRIPTION}}");
        eprintln!("Usage: {{PROJECT_NAME}} --stdio");
        std::process::exit(1);
    }

    info!("Starting {{PROJECT_NAME}} MCP server");
    let config = Arc::new(AppConfig::load());
    let registry = ToolRegistry::new(config);
    let server = McpServer::new(registry);
    server.run().await
}
