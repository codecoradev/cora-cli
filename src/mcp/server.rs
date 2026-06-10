//! MCP server — JSON-RPC 2.0 server over stdio transport.
//!
//! Reads JSON-RPC requests from stdin, dispatches to tool handlers,
//! writes responses to stdout.

use std::io::{self, Read, Write};

use tracing::{debug, error, info};

#[allow(unused_imports)]
use super::protocol::{
    InitializeResult, JsonRpcError, JsonRpcRequest, JsonRpcResponse, RequestId, ServerCapabilities,
    ServerInfo,
};
use super::tools;

const PROTOCOL_VERSION: &str = "2024-11-05";
const SERVER_NAME: &str = "cora-mcp";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Run the MCP server, reading from stdin and writing to stdout.
pub fn run_server() -> anyhow::Result<()> {
    info!("Starting cora MCP server on stdio");

    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    let mut buffer = String::new();

    // Read stdin byte-by-byte to handle multi-line JSON-RPC messages.
    // Line-based parsing breaks on pretty-printed JSON.
    // MCP spec: each message is a complete JSON object, optionally followed by newline.
    let mut brace_depth: i32 = 0;
    let mut in_string = false;
    let mut escape_next = false;

    for b in io::BufReader::new(io::stdin()).bytes() {
        let byte = b?;
        let ch = byte as char;

        if escape_next {
            escape_next = false;
            buffer.push(ch);
            continue;
        }

        if ch == '\\' && in_string {
            escape_next = true;
            buffer.push(ch);
            continue;
        }

        if ch == '"' {
            in_string = !in_string;
            buffer.push(ch);
            continue;
        }

        if !in_string {
            if ch == '{' {
                brace_depth += 1;
            } else if ch == '}' {
                brace_depth -= 1;
            }
        }

        buffer.push(ch);

        // Complete JSON object found when braces are balanced and buffer is non-empty
        if brace_depth == 0 && !buffer.trim().is_empty() {
            let trimmed = buffer.trim().to_string();
            buffer.clear();

            if trimmed.is_empty() {
                continue;
            }

            debug!(input = %trimmed, "received request");

            let request: JsonRpcRequest = match serde_json::from_str(&trimmed) {
                Ok(req) => req,
                Err(e) => {
                    error!(error = %e, "failed to parse request");
                    let err_resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: None,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32700,
                            message: format!("Parse error: {e}"),
                            data: None,
                        }),
                    };
                    write_response(&mut stdout_lock, &err_resp)?;
                    continue;
                }
            };

            let response = handle_request(&request);
            write_response(&mut stdout_lock, &response)?;
            stdout_lock.flush()?;

            // Exit on shutdown notification
            if request.method == "notifications/cancelled" || request.method == "shutdown" {
                info!("Shutting down MCP server");
                break;
            }
        }
    }

    Ok(())
}

fn handle_request(request: &JsonRpcRequest) -> JsonRpcResponse {
    match request.method.as_str() {
        "initialize" => handle_initialize(request),
        "initialized" => {
            // Notification — no response needed, but we send empty for JSON-RPC
            debug!("client initialized");
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id.clone(),
                result: Some(serde_json::json!({})),
                error: None,
            }
        }
        "tools/list" => handle_tools_list(request),
        "tools/call" => handle_tools_call(request),
        "ping" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id.clone(),
            result: Some(serde_json::json!({})),
            error: None,
        },
        _ => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id.clone(),
            result: None,
            error: Some(JsonRpcError {
                code: -32601,
                message: format!("Method not found: {}", request.method),
                data: None,
            }),
        },
    }
}

fn handle_initialize(request: &JsonRpcRequest) -> JsonRpcResponse {
    let result = InitializeResult {
        protocol_version: PROTOCOL_VERSION.to_string(),
        capabilities: ServerCapabilities {
            tools: serde_json::json!({}),
        },
        server_info: ServerInfo {
            name: SERVER_NAME.to_string(),
            version: SERVER_VERSION.to_string(),
        },
    };

    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: request.id.clone(),
        result: Some(serde_json::to_value(result).unwrap_or_default()),
        error: None,
    }
}

fn handle_tools_list(request: &JsonRpcRequest) -> JsonRpcResponse {
    let tools = tools::list_tools();
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: request.id.clone(),
        result: Some(serde_json::json!({ "tools": tools })),
        error: None,
    }
}

fn handle_tools_call(request: &JsonRpcRequest) -> JsonRpcResponse {
    let tool_name = request
        .params
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let args = request
        .params
        .get("arguments")
        .cloned()
        .unwrap_or(serde_json::json!({}));

    let result = tools::handle_tool_call(tool_name, &args);

    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: request.id.clone(),
        result: Some(serde_json::to_value(result).unwrap_or_default()),
        error: None,
    }
}

fn write_response(stdout: &mut io::StdoutLock, response: &JsonRpcResponse) -> anyhow::Result<()> {
    let json = serde_json::to_string(response)?;
    debug!(output = %json, "sending response");
    writeln!(stdout, "{json}")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_initialize_response() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(RequestId::Number(1)),
            method: "initialize".to_string(),
            params: serde_json::json!({}),
        };
        let resp = handle_request(&req);
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        assert_eq!(result["protocolVersion"], PROTOCOL_VERSION);
        assert_eq!(result["serverInfo"]["name"], SERVER_NAME);
    }

    #[test]
    fn handle_tools_list_response() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(RequestId::Number(2)),
            method: "tools/list".to_string(),
            params: serde_json::json!({}),
        };
        let resp = handle_request(&req);
        assert!(resp.result.is_some());
        let result = resp.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert!(!tools.is_empty());
    }

    #[test]
    fn handle_tools_call_list_rules() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(RequestId::Number(3)),
            method: "tools/call".to_string(),
            params: serde_json::json!({
                "name": "cora.list_rules",
                "arguments": {}
            }),
        };
        let resp = handle_request(&req);
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
    }

    #[test]
    fn handle_unknown_method() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(RequestId::Number(99)),
            method: "unknown/method".to_string(),
            params: serde_json::json!({}),
        };
        let resp = handle_request(&req);
        assert!(resp.error.is_some());
        assert_eq!(resp.error.unwrap().code, -32601);
    }

    #[test]
    fn handle_ping() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(RequestId::Number(4)),
            method: "ping".to_string(),
            params: serde_json::json!({}),
        };
        let resp = handle_request(&req);
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
    }
}
