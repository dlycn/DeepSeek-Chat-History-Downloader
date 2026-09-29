use serde_json::{Value, json};

use crate::analyzer;
use crate::ds_parser;
use crate::settings;
use crate::zhihu;

pub fn handle_init(args: &Value) -> String {
    let deepseek_dir = args["deepseek_dir"].as_str();
    let zhihu_cookie = args["zhihu_cookie"].as_str();
    let zhihu_xsrf = args["zhihu_xsrf"].as_str();

    settings::apply_and_save(deepseek_dir, zhihu_cookie, zhihu_xsrf)
}

pub fn handle_parse(args: &Value) -> String {
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

pub fn handle_session(args: &Value) -> String {
    let settings = settings::load_or_init();
    let dir = args["dir"].as_str().unwrap_or(&settings.deepseek_dir);
    let title = args["title"].as_str().unwrap_or("");

    if title.is_empty() {
        return "请提供会话标题关键词 (title 参数)。例如: zhihu_session(title=\"Rust\")"
            .to_string();
    } else if ds_parser::parse_session_detail(dir, title).is_none() {
        return format!("未找到标题包含 \"{}\" 的会话。请尝试其他关键词。", title);
    } else {
        let detail = ds_parser::parse_session_detail(dir, title).unwrap();
        return analyzer::format_session_detail(&detail);
        
    }
}

pub async fn handle_daily(args: &Value) -> String {
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
            let api = zhihu::Api::run(cookie, xsrf);
            let zhihu_questions = api.get_question_list().await;
            analyzer::format_zhihu_for_ai(&zhihu_questions, &summaries, total_count, limit)
        }
        _ => {
            let mut output = String::new();
            output.push_str("## 未配置知乎凭据\n\n");
            output.push_str("请先通过 `zhihu_init` 工具配置 Cookie 后重试。\n\n");
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

pub fn tool_definitions() -> Vec<Value> {
    vec![
        tool("zhihu_init", "配置知乎凭据（Cookie 和 x-xsrftoken）。保存到 settings.toml")
            .prop("deepseek_dir", "string", "DeepSeek 对话 JSON 目录路径")
            .prop("zhihu_cookie", "string", "知乎 Cookie 完整值（从浏览器 F12 → Network 复制）")
            .prop("zhihu_xsrf", "string", "知乎 x-xsrftoken（Cookie 中 _xsrf= 的值）")
            .build(),
        tool("zhihu_parse", "扫描 DS 对话目录，输出所有会话摘要（标题+提问示例+统计）。不做关键词匹配，交给 AI 分析。默认展示最近 50 个会话")
            .prop("dir", "string", "DS 对话 JSON 目录，默认使用 settings.toml 中的 deepseek_dir")
            .prop("limit", "integer", "展示会话数上限，默认 50")
            .build(),
        tool("zhihu_session", "按标题关键词搜索并返回某个会话的完整详情（含所有对话轮次，每段截断至 500 字）")
            .prop("dir", "string", "DS 对话 JSON 目录")
            .prop("title", "string", "会话标题关键词（模糊匹配）")
            .required(&["title"])
            .build(),
        tool("zhihu_daily", "完整日常：导出会话摘要 + 拉取知乎推荐问题列表，交给 AI 做兴趣匹配和回答生成。需先通过 zhihu_init 配置知乎凭据")
            .prop("dir", "string", "DS 对话 JSON 目录，默认使用 settings.toml 中的 deepseek_dir")
            .prop("limit", "integer", "展示会话数上限，默认 50")
            .build(),
    ]
}

fn tool(name: &'static str, description: &'static str) -> ToolDef {
    ToolDef { name, description, properties: vec![], required: vec![] }
}

struct ToolDef {
    name: &'static str,
    description: &'static str,
    properties: Vec<(Value, Value)>,
    required: Vec<&'static str>,
}

impl ToolDef {
    fn prop(mut self, name: &'static str, typ: &'static str, desc: &'static str) -> Self {
        let prop = json!({ "type": typ, "description": desc });
        self.properties.push((Value::String(name.to_string()), prop));
        self
    }

    fn required(mut self, fields: &[&'static str]) -> Self {
        self.required = fields.to_vec();
        self
    }

    fn build(self) -> Value {
        let mut schema = json!({
            "type": "object",
            "properties": {}
        });
        for (k, v) in &self.properties {
            schema["properties"][k.as_str().unwrap()] = v.clone();
        }
        if !self.required.is_empty() {
            schema["required"] = json!(self.required);
        }
        json!({
            "name": self.name,
            "description": self.description,
            "inputSchema": schema
        })
    }
}