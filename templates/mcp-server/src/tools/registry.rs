use anyhow::{bail, Result};
use serde_json::Value;
use std::sync::Arc;

use crate::config::AppConfig;
use crate::mcp::protocol::{ToolDefinition, ToolResult};
use crate::tools::health_check;

pub struct ToolRegistry {
    config: Arc<AppConfig>,
}

impl ToolRegistry {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }

    pub fn list_tools(&self) -> Vec<ToolDefinition> {
        vec![Self::def_health_check()]
    }

    pub async fn call_tool(&self, name: &str, args: Value) -> Result<ToolResult> {
        match name {
            "health_check" => health_check::health_check(&self.config, args).await,
            _ => bail!("Unknown tool: {name}"),
        }
    }

    fn def_health_check() -> ToolDefinition {
        ToolDefinition {
            name: "health_check".into(),
            description: "Check server health and return version info".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        }
    }
}
