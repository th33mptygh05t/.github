use anyhow::Result;
use serde_json::Value;

use crate::config::AppConfig;
use crate::mcp::protocol::ToolResult;

pub async fn health_check(_config: &AppConfig, _args: Value) -> Result<ToolResult> {
    let status = format!(
        "server: {{PROJECT_NAME}}\nversion: {version}\nstatus: healthy",
        version = env!("CARGO_PKG_VERSION"),
    );

    Ok(ToolResult::success(status))
}
