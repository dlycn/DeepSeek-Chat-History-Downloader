use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::sync::Mutex;

static LOGGER: std::sync::LazyLock<Mutex<File>> =
    std::sync::LazyLock::new(|| Mutex::new(open_log_file()));

const LOG_FILE: &str = "mcp.log";
const MAX_RESPONSE_LEN: usize = 600;

fn open_log_file() -> File {
    if fs::metadata(LOG_FILE).ok().map_or(false, |m| m.len() > 5 * 1024 * 1024) {
        let _ = fs::remove_file(LOG_FILE);
    }

    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .expect("无法创建日志文件 mcp.log");

    let now = chrono_diy();
    writeln!(
        f,
        "\n═════════════════════════════════════════════════"
    )
    .ok();
    writeln!(f, "  MCP Session 开始 — {}", now).ok();
    writeln!(
        f,
        "═════════════════════════════════════════════════\n"
    )
    .ok();
    f.flush().ok();

    f
}

pub fn log_input(raw_json: &str) {
    let now = chrono_diy();
    let mut f = LOGGER.lock().unwrap();
    writeln!(f, "[{}] ⬅ IN  ──────────────────────", now).ok();
    writeln!(f, "{}", raw_json).ok();
    writeln!(f).ok();
    f.flush().ok();
}

pub fn log_output(raw_json: &str) {
    let now = chrono_diy();
    let mut f = LOGGER.lock().unwrap();
    writeln!(f, "[{}] ➡ OUT ──────────────────────", now).ok();

    if raw_json.len() > MAX_RESPONSE_LEN {
        let truncated: String = raw_json
            .chars()
            .take(MAX_RESPONSE_LEN)
            .collect();
        writeln!(f, "{}...", truncated).ok();
        writeln!(
            f,
            "   (已截断，完整长度: {} 字)",
            raw_json.len()
        )
        .ok();
    } else {
        writeln!(f, "{}", raw_json).ok();
    }

    writeln!(f).ok();
    f.flush().ok();
}

fn chrono_diy() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let days = secs / 86400;
    let time = secs % 86400;
    let h = time / 3600;
    let m = (time % 3600) / 60;
    let s = time % 60;
    format!("第{}天 {:02}:{:02}:{:02}", days, h, m, s)
}