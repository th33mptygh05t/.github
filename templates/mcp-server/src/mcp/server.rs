use anyhow::Result;
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{debug, error, info};

use crate::mcp::protocol::*;
use crate::tools::registry::ToolRegistry;

pub struct McpServer {
    registry: ToolRegistry,
}

impl McpServer {
    pub fn new(registry: ToolRegistry) -> Self {
        Self { registry }
    }

    pub async fn run(self) -> Result<()> {
        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        info!("{{PROJECT_NAME}} MCP server starting on stdio");

        while let Some(line) = lines.next_line().await? {
            let line = line.trim().to_string();
            if line.is_empty() {
                continue;
            }

            debug!(input = %line, "Received request");

            let response = self.handle_message(&line).await;

            if let Some(resp) = response {
                let json = serde_json::to_string(&resp)?;
                debug!(output = %json, "Sending response");
                stdout.write_all(json.as_bytes()).await?;
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
            }
        }

        info!("{{PROJECT_NAME}} MCP server shutting down");
        Ok(())
    }

    async fn handle_message(&self, line: &str) -> Option<JsonRpcResponse> {
        let request: JsonRpcRequest = match serde_json::from_str(line) {
            Ok(req) => req,
            Err(e) => {
                error!(error = %e, "Failed to parse JSON-RPC request");
                return Some(JsonRpcResponse::error(
                    None,
                    -32700,
                    format!("Parse error: {e}"),
                ));
            }
        };

        let id = request.id.clone();
        let method = request.method.as_str();

        match method {
            "initialize" => Some(self.handle_initialize(id, request.params)),
            "notifications/initialized" => {
                info!("Client initialized notification received");
                None
            }
            "tools/list" => Some(self.handle_tools_list(id)),
            "tools/call" => Some(self.handle_tools_call(id, request.params).await),
            "ping" => Some(JsonRpcResponse::success(id, serde_json::json!({}))),
            _ => Some(JsonRpcResponse::error(
                id,
                -32601,
                format!("Method not found: {method}"),
            )),
        }
    }

    fn handle_initialize(&self, id: Option<Value>, params: Option<Value>) -> JsonRpcResponse {
        if let Some(params) = params {
            match serde_json::from_value::<InitializeParams>(params) {
                Ok(init_params) => {
                    info!(
                        client_version = %init_params.protocol_version,
                        "Initialize request received"
                    );
                }
                Err(e) => {
                    error!(error = %e, "Failed to parse initialize params");
                }
            }
        }

        let result = InitializeResult {
            protocol_version: PROTOCOL_VERSION.to_string(),
            capabilities: ServerCapabilities {
                tools: ToolsCapability {
                    list_changed: false,
                },
            },
            server_info: ServerInfo {
                name: SERVER_NAME.to_string(),
                version: SERVER_VERSION.to_string(),
            },
        };

        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
    }

    fn handle_tools_list(&self, id: Option<Value>) -> JsonRpcResponse {
        let tools = self.registry.list_tools();
        let result = serde_json::json!({ "tools": tools });
        JsonRpcResponse::success(id, result)
    }

    async fn handle_tools_call(&self, id: Option<Value>, params: Option<Value>) -> JsonRpcResponse {
        let params = match params {
            Some(p) => p,
            None => {
                return JsonRpcResponse::error(id, -32602, "Missing params".to_string());
            }
        };

        let call_params: ToolCallParams = match serde_json::from_value(params) {
            Ok(p) => p,
            Err(e) => {
                return JsonRpcResponse::error(
                    id,
                    -32602,
                    format!("Invalid tool call params: {e}"),
                );
            }
        };

        let args = call_params.arguments.unwrap_or(serde_json::json!({}));
        info!(tool = %call_params.name, "Calling tool");

        let result = self.registry.call_tool(&call_params.name, args).await;
        match result {
            Ok(tool_result) => {
                JsonRpcResponse::success(id, serde_json::to_value(tool_result).unwrap())
            }
            Err(e) => {
                let tool_result = ToolResult::error(format!("Tool execution error: {e}"));
                JsonRpcResponse::success(id, serde_json::to_value(tool_result).unwrap())
            }
        }
    }
}
