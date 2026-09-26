// MCP Server: zhihu-daily
// 协议: Model Context Protocol (MCP)
//   - 官方站点: https://modelcontextprotocol.io
//   - 协议规范: https://spec.modelcontextprotocol.io
//   - GitHub:   https://github.com/modelcontextprotocol/specification
// 传输: stdio (JSON-RPC 2.0)
// 版本: 2024-11-05
mod analyzer;
mod config;
mod ds_parser;
mod logger;
mod settings;
mod types;
mod zhihu;

use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

#[tokio::main]
async fn main() {
    let _lock = acquire_singleton_lock();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        if line.is_empty() {
            continue;
        }

        let request: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[mcp] JSON parse error: {}", e);
                continue;
            }
        };

        logger::log_input(&line);

        if let Some(response) = handle_request(&request).await {
            if let Ok(response_str) = serde_json::to_string(&response) {
                logger::log_output(&response_str);
                writeln!(stdout, "{}", response_str).unwrap();
                stdout.flush().unwrap();
            }
        }
    }
}

async fn handle_request(request: &Value) -> Option<Value> {
    let method = request["method"].as_str().unwrap_or("");
    let id = request.get("id").cloned().unwrap_or(Value::Null);
    let is_notification = request.get("id").is_none();

    let result = match method {
        "initialize" => Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "zhihu-daily",
                    "version": "1.0.0"
                }
            }
        })),

        _ if is_notification => None,

        "tools/list" => Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "tools": [
                    {
                        "name": "zhihu_init",
                        "description": "配置知乎凭据（Cookie 和 x-xsrftoken）。保存到 settings.toml",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "deepseek_dir": {
                                    "type": "string",
                                    "description": "DeepSeek 对话 JSON 目录路径"
                                },
                                "zhihu_cookie": {
                                    "type": "string",
                                    "description": "知乎 Cookie 完整值（从浏览器 F12 → Network 复制）"
                                },
                                "zhihu_xsrf": {
                                    "type": "string",
                                    "description": "知乎 x-xsrftoken（Cookie 中 _xsrf= 的值）"
                                }
                            }
                        }
                    },
                    {
                        "name": "zhihu_parse",
                        "description": "扫描 DS 对话目录，输出所有会话摘要（标题+提问示例+统计）。不做关键词匹配，交给 AI 分析。默认展示最近 50 个会话",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "dir": {
                                    "type": "string",
                                    "description": "DS 对话 JSON 目录，默认使用 settings.toml 中的 deepseek_dir"
                                },
                                "limit": {
                                    "type": "integer",
                                    "description": "展示会话数上限，默认 50"
                                }
                            }
                        }
                    },
                    {
                        "name": "zhihu_session",
                        "description": "按标题关键词搜索并返回某个会话的完整详情（含所有对话轮次，每段截断至 500 字）",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "dir": {
                                    "type": "string",
                                    "description": "DS 对话 JSON 目录"
                                },
                                "title": {
                                    "type": "string",
                                    "description": "会话标题关键词（模糊匹配）"
                                }
                            },
                            "required": ["title"]
                        }
                    },
                    {
                        "name": "zhihu_daily",
                        "description": "完整日常：导出会话摘要 + 拉取知乎推荐问题列表，交给 AI 做兴趣匹配和回答生成。需先通过 zhihu_init 配置知乎凭据",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "dir": {
                                    "type": "string",
                                    "description": "DS 对话 JSON 目录，默认使用 settings.toml 中的 deepseek_dir"
                                },
                                "limit": {
                                    "type": "integer",
                                    "description": "展示会话数上限，默认 50"
                                }
                            }
                        }
                    }
                ]
            }
        })),

        "tools/call" => {
            let tool_name = request["params"]["name"].as_str().unwrap_or("");
            let tool_args = &request["params"]["arguments"];

            let text = match tool_name {
                "zhihu_init" => handle_init(tool_args),
                "zhihu_parse" => handle_parse(tool_args),
                "zhihu_session" => handle_session(tool_args),
                "zhihu_daily" => handle_daily(tool_args).await,
                _ => format!("未知工具: {}", tool_name),
            };

            Some(json!({
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
            }))
        }

        _ => Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": -32601,
                "message": format!("不支持的方法: {}", method)
            }
        })),
    };

    result
}

fn handle_init(args: &Value) -> String {
    let deepseek_dir = args["deepseek_dir"].as_str();
    let zhihu_cookie = args["zhihu_cookie"].as_str();
    let zhihu_xsrf = args["zhihu_xsrf"].as_str();

    settings::apply_and_save(deepseek_dir, zhihu_cookie, zhihu_xsrf)
}

fn handle_parse(args: &Value) -> String {
    let settings = settings::load_or_init();
    let dir = args["dir"].as_str().unwrap_or(&settings.deepseek_dir);
    let limit = args["limit"].as_u64().unwrap_or(50) as usize;

    let total_count = ds_parser::count_all_sessions(dir);
    let summaries = ds_parser::parse_dir_summaries(dir, limit);

    if summaries.is_empty() {
        return format!(
            "未在目录 `{}` 中找到 DS 对话 JSON 文件。请确认路径正确。",
            dir
        );
    }

    analyzer::format_session_list(&summaries, total_count, limit)
}

fn handle_session(args: &Value) -> String {
    let settings = settings::load_or_init();
    let dir = args["dir"].as_str().unwrap_or(&settings.deepseek_dir);
    let title = args["title"].as_str().unwrap_or("");

    if title.is_empty() {
        return "请提供会话标题关键词 (title 参数)。例如: zhihu_session(title=\"Rust\")".to_string();
    }

    match ds_parser::parse_session_detail(dir, title) {
        Some(detail) => analyzer::format_session_detail(&detail),
        None => format!("未找到标题包含 \"{}\" 的会话。请尝试其他关键词。", title),
    }
}

async fn handle_daily(args: &Value) -> String {
    let settings = settings::load_or_init();
    let dir = args["dir"].as_str().unwrap_or(&settings.deepseek_dir);
    let limit = args["limit"].as_u64().unwrap_or(50) as usize;

    let total_count = ds_parser::count_all_sessions(dir);
    let summaries = ds_parser::parse_dir_summaries(dir, limit);

    if summaries.is_empty() {
        return format!(
            "未在目录 `{}` 中找到 DS 对话 JSON 文件。请确认路径正确。",
            dir
        );
    }

    match (&settings.zhihu_cookie, &settings.zhihu_xsrf) {
        (cookie, xsrf) if !cookie.is_empty() && !xsrf.is_empty() => {
            let headers = config::build_headers(cookie, xsrf);
            let urls = zhihu::init_urls();
            let body = zhihu::get_from_id(urls.get("question").unwrap(), headers).await;
            let zhihu_questions = zhihu::get_question_list(body);

            analyzer::format_zhihu_for_ai(&zhihu_questions, &summaries, total_count, limit)
        }
        _ => {
            let mut output = String::new();
            output.push_str("## 未配置知乎凭据\n\n");
            output.push_str(
                "请先通过 `zhihu_init` 工具配置 Cookie 后重试。\n\n",
            );
            output.push_str("以下仅展示 DS 会话分析:\n\n");
            output.push_str(&analyzer::format_session_list(
                &summaries,
                total_count,
                limit,
            ));
            output
        }
    }
}

use std::fs;

fn lock_path() -> std::path::PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("mcp.lock")
}

fn acquire_singleton_lock() -> Option<fs::File> {
    let path = lock_path();
    let file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path);

    match file {
        Ok(f) => {
            let _ = fs::write(&path, std::process::id().to_string());
            Some(f)
        }
        Err(_) => {
            let meta = fs::metadata(&path);
            let is_stale = meta.as_ref().map_or(true, |m| {
                m.modified().map_or(true, |t| {
                    t.elapsed().map_or(true, |d| d.as_secs() > 300)
                })
            });

            if is_stale {
                let _ = fs::remove_file(&path);
                match fs::OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(&path)
                {
                    Ok(f) => {
                        let _ = fs::write(&path, std::process::id().to_string());
                        return Some(f);
                    }
                    Err(e) => {
                        eprintln!("[fatal] 无法创建锁文件: {}", e);
                        std::process::exit(1);
                    }
                }
            }

            eprintln!("[fatal] 已有 MPC_zhihu.exe 正在运行。锁文件: {}", path.display());
            eprintln!("[fatal] 如需强制启动，请删除 {} 后重试", path.display());
            std::process::exit(1);
        }
    }
}