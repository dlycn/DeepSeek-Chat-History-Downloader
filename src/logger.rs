use std::path::PathBuf;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};



pub struct McpLogger {
    _guard: tracing_appender::non_blocking::WorkerGuard,
    cutlen: usize,
}

impl McpLogger {
    pub fn init(file: &str) -> Self {
        Self {
            _guard: Self::init_tracing("logs", file),
            cutlen: usize::MAX,
        }
    }

    pub fn set_cutlen(&mut self, len: usize) {
        self.cutlen = len;
    }

    pub fn log(&self, msg: &str) {
        tracing::info!("{msg}");
    }

    pub fn input(&self, raw_json: &str) {
        tracing::info!("IN: {raw_json}");
    }

    pub fn output(&self, raw_json: &str) {
        // 按 char 安全截断，找到字节位置
        let end = raw_json
            .char_indices()
            .nth(self.cutlen)
            .map(|(i, _)| i);

        if end.is_some() {
            let i = end.unwrap();
            tracing::info!(
                "OUT: {}...(all chars num: {})",
                &raw_json[..i],
                raw_json.chars().count(),
            )
        } else {
            tracing::info!("OUT: {raw_json}");
        }
    }

    fn init_tracing(dir: &str, file: &str) -> tracing_appender::non_blocking::WorkerGuard {
        let main_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
        let pathdir = main_dir.join(dir);
        let file_appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix(file)
            .filename_suffix("log")
            .max_log_files(7)
            .build(pathdir)
            .unwrap();

        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        tracing_subscriber::registry()
            .with(EnvFilter::new("MCP_zhihu=info"))
            .with(
                fmt::layer()
                    .with_ansi(false)
                    .with_target(false)
                    .with_writer(non_blocking)
                    .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339()),
            )
            .init();

        guard
    }
}

