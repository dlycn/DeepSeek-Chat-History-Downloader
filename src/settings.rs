use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn base_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

fn settings_path() -> PathBuf {
    base_dir().join("settings.toml")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    #[serde(default = "default_deepseek_dir")]
    pub deepseek_dir: String,

    #[serde(default)]
    pub zhihu_cookie: String,

    #[serde(default)]
    pub zhihu_xsrf: String,
}

fn default_deepseek_dir() -> String {
    base_dir()
        .join("deepseek")
        .to_string_lossy()
        .to_string()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            deepseek_dir: default_deepseek_dir(),
            zhihu_cookie: String::new(),
            zhihu_xsrf: String::new(),
        }
    }
}

pub fn load_or_init() -> Settings {
    let path = settings_path();
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(s) => {
                    return s;
                }
                Err(e) => {
                    eprintln!(
                        "[config] {} 解析失败: {}",
                        path.display(),
                        e
                    )
                }
            },
            Err(e) => eprintln!("[config] 读取 {} 失败: {}", path.display(), e),
        }
    }

    let defaults = Settings::default();
    match toml::to_string_pretty(&defaults) {
        Ok(content) => {
            if let Err(e) = fs::write(&path, &content) {
                eprintln!("[config] 写入默认配置 {} 失败: {}", path.display(), e);
            } else {
                eprintln!("[config] 已创建默认配置文件: {}", path.display());
            }
        }
        Err(e) => eprintln!("[config] 序列化默认配置失败: {}", e),
    }
    defaults
}

pub fn save_settings(settings: &Settings) -> String {
    let path = settings_path();
    let toml_str = toml::to_string_pretty(settings).unwrap();
    fs::write(&path, &toml_str).unwrap();
    path.to_string_lossy().to_string()
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
        settings.zhihu_cookie = cookie.to_string();
    }
    if let Some(xsrf) = zhihu_xsrf {
        settings.zhihu_xsrf = xsrf.to_string();
    }

    let file = save_settings(&settings);

    let mut report = format!("配置已保存到 {}\n", file);
    report.push_str(&format!("- deepseek_dir: {}\n", settings.deepseek_dir));
    report.push_str(&format!(
        "- zhihu_cookie: {}\n",
        if !settings.zhihu_cookie.is_empty() {
            "已配置"
        } else {
            "未配置"
        }
    ));
    report.push_str(&format!(
        "- zhihu_xsrf: {}",
        if !settings.zhihu_xsrf.is_empty() {
            "已配置"
        } else {
            "未配置"
        }
    ));
    report
}