use serde::{Deserialize, Serialize};
use std::fs;
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
                    return s;
                }
                Err(e) => eprintln!("[config] {} 解析失败: {}", SETTINGS_FILE, e),
            },
            Err(e) => eprintln!("[config] 读取 {} 失败: {}", SETTINGS_FILE, e),
        }
    }

    Settings::default()
}

pub fn save_settings(settings: &Settings) -> String {
    let toml_str = toml::to_string_pretty(settings).unwrap();
    fs::write(SETTINGS_FILE, &toml_str).unwrap();
    SETTINGS_FILE.to_string()
}

pub fn apply_and_save(
    deepseek_dir: Option<&str>,
    zhihu_cookie: Option<&str>,
    zhihu_xsrf: Option<&str>,
) -> String {
    let mut settings = load_or_init();

    if let Some(dir) = deepseek_dir {
        settings.deepseek_dir = dir.to_string();
    }
    if let Some(cookie) = zhihu_cookie {
        if cookie.is_empty() {
            settings.zhihu_cookie = None;
        } else {
            settings.zhihu_cookie = Some(cookie.to_string());
        }
    }
    if let Some(xsrf) = zhihu_xsrf {
        if xsrf.is_empty() {
            settings.zhihu_xsrf = None;
        } else {
            settings.zhihu_xsrf = Some(xsrf.to_string());
        }
    }

    let file = save_settings(&settings);

    let mut report = format!("配置已保存到 {}\n", file);
    report.push_str(&format!("- deepseek_dir: {}\n", settings.deepseek_dir));
    report.push_str(&format!(
        "- zhihu_cookie: {}\n",
        if settings.zhihu_cookie.is_some() {
            "已配置"
        } else {
            "未配置"
        }
    ));
    report.push_str(&format!(
        "- zhihu_xsrf: {}",
        if settings.zhihu_xsrf.is_some() {
            "已配置"
        } else {
            "未配置"
        }
    ));

    if settings.zhihu_cookie.is_none() {
        report.push_str("\n⚠ 未配置知乎凭据，daily 将仅做本地分析。");
    }

    report
}