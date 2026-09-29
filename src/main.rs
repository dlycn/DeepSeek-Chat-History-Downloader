mod analyzer;
mod ds_parser;
mod handlers;
mod logger;
mod mcp;
mod settings;
mod types;
mod zhihu;

use serde_json::Value;
use std::io::{self, BufRead, Write};
// MCP Server: zhihu-daily
// 协议: Model Context Protocol (MCP)
//   - 官方站点: https://modelcontextprotocol.io
//   - 协议规范: https://spec.modelcontextprotocol.io
//   - GitHub:   https://github.com/modelcontextprotocol/specification
// 传输: stdio (JSON-RPC 2.0)
// 版本: 2024-11-05
#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut logger = logger::McpLogger::init("mcp");
    logger.set_cutlen(1000);
    logger.log("logger initialize");

    let mcp = mcp::McpServer::new("zhihu-daily", "1.0.0")
        .with_tools(handlers::tool_definitions());

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                tracing::error!("Error reading line: {}", e);
                continue;
            }
        };
        if line.is_empty() {
            continue;
        }

        let request: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("JSON parse error: {}", e);
                continue;
            }
        };

        logger.input(&line);

        let id = request.get("id");
        let method = request["method"].as_str().unwrap_or("");
        let params = request.get("params");

        if let Some(response) = mcp.route(id, method, params).await {
            if let Ok(response_str) = serde_json::to_string(&response) {
                logger.output(&response_str);
                writeln!(stdout, "{}", response_str).unwrap();
                stdout.flush().unwrap();
            }
        }
    }
}