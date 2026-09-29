use serde_json::{Value, json};

use crate::handlers;

pub struct McpServer {
    name: &'static str,
    version: &'static str,
    tools: Vec<Value>,
}

impl McpServer {
    pub fn new(name: &'static str, version: &'static str) -> Self {
        Self {
            name,
            version,
            tools: Vec::new(),
        }
    }

    pub fn with_tools(mut self, tools: Vec<Value>) -> Self {
        self.tools = tools;
        self
    }

    pub async fn route(
        &self,
        id: Option<&Value>,
        method: &str,
        params: Option<&Value>,
    ) -> Option<Value> {
        let id = id?;
        if id.is_null() {
            return None;
        }

        match method {
            "initialize" => Some(Self::init_response(id, self.name, self.version)),

            "ping" => Some(Self::ping_response(id)),

            "tools/list" => Some(Self::tools_list_response(id, &self.tools)),

            "tools/call" => {
                let params = params?;
                let tool_name = params["name"].as_str().unwrap_or("");
                let tool_args = &params["arguments"];
                let text = dispatch_tool(tool_name, tool_args).await;
                Some(Self::tool_result_response(id, &text))
            }

            _ => Some(Self::error_response(id, method)),
        }
    }

    fn init_response(id: &Value, name: &str, version: &str) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": name,
                    "version": version
                }
            }
        })
    }

    fn ping_response(id: &Value) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {}
        })
    }

    fn tools_list_response(id: &Value, tools: &[Value]) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "tools": tools
            }
        })
    }

    fn tool_result_response(id: &Value, text: &str) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "content": [
                    {
                        "type": "text",
                        "text": text
                    }
                ]
            }
        })
    }

    fn error_response(id: &Value, method: &str) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": -32601,
                "message": format!("不支持的方法: {}", method)
            }
        })
    }
}

async fn dispatch_tool(name: &str, args: &Value) -> String {
    match name {
        "zhihu_init" => handlers::handle_init(args),
        "zhihu_parse" => handlers::handle_parse(args),
        "zhihu_session" => handlers::handle_session(args),
        "zhihu_daily" => handlers::handle_daily(args).await,
        _ => format!("未知工具: {}", name),
    }
}