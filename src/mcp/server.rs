use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use tracing::info;

use super::tools::ToolRegistry;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

pub async fn start_mcp_server(stealth: bool, proxy: Option<String>) -> anyhow::Result<()> {
    info!("NexusMCP MCP Server starting...");

    let mut registry = ToolRegistry::new();
    registry.register_tools(stealth, proxy);

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(e) => {
                tracing::error!("Failed to parse request: {}", e);
                continue;
            }
        };

        if let Some(response) = handle_request(&request, &mut registry).await {
            let response_json = serde_json::to_string(&response)?;
            writeln!(stdout, "{}", response_json)?;
            stdout.flush()?;
        }
    }

    Ok(())
}

async fn handle_request(
    request: &JsonRpcRequest,
    registry: &mut ToolRegistry,
) -> Option<JsonRpcResponse> {
    // If request.id is None, it is a JSON-RPC notification. We must NOT respond.
    if request.id.is_none() {
        info!("Received notification method: {}", request.method);
        return None;
    }

    let response = match request.method.as_str() {
        "initialize" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id.clone(),
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "nexusmcp",
                    "version": "0.1.0"
                }
            })),
            error: None,
        },

        "tools/list" => {
            let tools = registry.list_tools();
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id.clone(),
                result: Some(json!({ "tools": tools })),
                error: None,
            }
        }

        "tools/call" => {
            if let Some(params) = &request.params {
                if let Some(name) = params.get("name").and_then(|v| v.as_str()) {
                    let args = params.get("arguments").cloned().unwrap_or(json!({}));

                    match registry.call_tool(name, args).await {
                        Ok(result) => JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id.clone(),
                            result: Some(json!({
                                "content": [{
                                    "type": "text",
                                    "text": result
                                }]
                            })),
                            error: None,
                        },
                        Err(e) => {
                            // Protocol compliance: tool call execution failures must be returned 
                            // in a successful JSON-RPC response with isError: true inside result
                            JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: request.id.clone(),
                                result: Some(json!({
                                    "content": [{
                                        "type": "text",
                                        "text": format!("Error executing tool: {}", e)
                                    }],
                                    "isError": true
                                })),
                                error: None,
                            }
                        }
                    }
                } else {
                    error_response(request, -32602, "Missing tool name")
                }
            } else {
                error_response(request, -32602, "Missing params")
            }
        }

        _ => error_response(request, -32601, "Method not found"),
    };

    Some(response)
}

fn error_response(request: &JsonRpcRequest, code: i32, message: &str) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: request.id.clone(),
        result: None,
        error: Some(JsonRpcError {
            code,
            message: message.to_string(),
            data: None,
        }),
    }
}