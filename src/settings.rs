use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const SETTINGS_FILE: &str = "settings.toml";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    #[serde(default = "default_deepseek_dir")]
    pub deepseek_dir: String,

    #[serde(default)]
    pub zhihu_cookie: Option<String>,

    #[serde(default)]
    pub zhihu_xsrf: Option<String>,
}

fn default_deepseek_dir() -> String {
    "deepseek".to_string()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            deepseek_dir: default_deepseek_dir(),
            zhihu_cookie: None,
            zhihu_xsrf: None,
        }
    }
}

pub fn load_or_init() -> Settings {
    if Path::new(SETTINGS_FILE).exists() {
        match fs::read_to_string(SETTINGS_FILE) {
            Ok(content) => match toml::from_str(&content) {
                Ok(s) => {
                    println!("[config] 已加载 {}", SETTINGS_FILE);
                    return s;
                }
                Err(e) => eprintln!("[config] {} 解析失败: {}, 重新配置", SETTINGS_FILE, e),
            },
            Err(e) => eprintln!("[config] 读取 {} 失败: {}", SETTINGS_FILE, e),
        }
    }

    println!("[config] 未找到 {}, 使用默认值", SETTINGS_FILE);
    println!("[config] 提示: 运行 `init` 命令配置知乎凭据以启用 daily 功能\n");
    Settings::default()
}

pub fn run_init() -> Settings {
    println!("\n═══════════════════════════════════");
    println!("  知乎日常助手 — 首次配置");
    println!("═══════════════════════════════════\n");

    let mut settings = if Path::new(SETTINGS_FILE).exists() {
        match fs::read_to_string(SETTINGS_FILE) {
            Ok(content) => {
                let s: Settings = toml::from_str(&content).unwrap_or_default();
                println!("[config] 检测到已有配置，将基于现有值更新\n");
                s
            }
            Err(_) => Settings::default(),
        }
    } else {
        Settings::default()
    };

    settings.deepseek_dir = prompt(
        "DeepSeek 对话 JSON 目录",
        &settings.deepseek_dir,
    );

    let cookie_default = settings.zhihu_cookie.as_deref().unwrap_or("");
    let cookie_input = prompt_multiline(
        "知乎 Cookie (浏览器F12 → Network → 复制cookie请求头完整值，留空跳过)",
        cookie_default,
    );
    settings.zhihu_cookie = if cookie_input.is_empty() {
        None
    } else {
        Some(cookie_input)
    };

    let xsrf_default = settings.zhihu_xsrf.as_deref().unwrap_or("");
    let xsrf_input = prompt(
        "知乎 x-xsrftoken (Cookie 中 _xsrf= 字段的值，留空跳过)",
        xsrf_default,
    );
    settings.zhihu_xsrf = if xsrf_input.is_empty() {
        None
    } else {
        Some(xsrf_input)
    };

    let toml_str = toml::to_string_pretty(&settings).unwrap();
    fs::write(SETTINGS_FILE, &toml_str).unwrap();
    println!("\n[config] 配置已保存到 {}\n", SETTINGS_FILE);

    if settings.zhihu_cookie.is_none() {
        println!("[config] 未配置知乎凭据，`daily` 命令将仅做本地分析。");
        println!("[config] 之后可随时运行 `init` 补充凭据。\n");
    }

    settings
}

fn prompt(label: &str, default: &str) -> String {
    if default.is_empty() {
        print!("{}: ", label);
    } else {
        print!("{} [{}]: ", label, default);
    }
    io::stdout().flush().ok();

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let input = input.trim().to_string();

    if input.is_empty() && !default.is_empty() {
        default.to_string()
    } else {
        input
    }
}

fn prompt_multiline(label: &str, default: &str) -> String {
    println!("{}", label);
    if !default.is_empty() {
        println!("(当前值长度: {} 字符)", default.len());
    }
    println!("粘贴后按 Enter 输入空行结束:");
    io::stdout().flush().ok();

    let mut lines: Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).ok();
        let trimmed = line.trim().to_string();
        if trimmed.is_empty() {
            break;
        }
        lines.push(trimmed);
    }

    let result = lines.join("");
    if result.is_empty() && !default.is_empty() {
        println!("(使用现有值)");
        default.to_string()
    } else {
        result
    }
}