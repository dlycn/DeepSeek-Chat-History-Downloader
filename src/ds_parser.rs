use std::fs;
use std::path::Path;

use crate::types::{ChatHistory, ConversationTurn, SessionDetail, SessionSummary};

pub fn parse_dir_summaries(dir: &str, limit: usize) -> Vec<SessionSummary> {
    let mut summaries = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        eprintln!("无法读取目录: {}", dir);
        return summaries;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(true, |e| e != "json") {
            continue;
        }
        if let Some(summary) = parse_file_summary(&path) {
            summaries.push(summary);
        }
    }

    summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    if summaries.len() > limit {
        summaries.truncate(limit);
    }

    summaries
}

pub fn parse_session_detail(dir: &str, title_keyword: &str) -> Option<SessionDetail> {
    let Ok(entries) = fs::read_dir(dir) else {
        return None;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(true, |e| e != "json") {
            continue;
        }
        let content = fs::read_to_string(&path).ok()?;
        let history: ChatHistory = serde_json::from_str(&content).ok()?;
        let session_title = history
            .data
            .biz_data
            .chat_session
            .title
            .clone()
            .unwrap_or_else(|| "未命名".to_string());

        if session_title.contains(title_keyword) {
            let updated = format_timestamp(history.data.biz_data.chat_session.updated_at);
            let turns: Vec<ConversationTurn> = history
                .data
                .biz_data
                .chat_messages
                .iter()
                .filter(|m| m.role == "USER" || m.role == "ASSISTANT")
                .filter_map(|m| {
                    let content: String = m
                        .fragments
                        .iter()
                        .filter(|f| f.frag_type == "REQUEST" || f.frag_type == "TEXT")
                        .map(|f| f.content.as_str())
                        .collect::<Vec<_>>()
                        .join("\n");

                    if content.trim().is_empty() {
                        return None;
                    }

                    Some(ConversationTurn {
                        role: m.role.clone(),
                        content: truncate(&content, 500),
                    })
                })
                .collect();

            return Some(SessionDetail {
                title: session_title,
                updated_at: updated,
                turns,
            });
        }
    }

    None
}

pub fn count_all_sessions(dir: &str) -> usize {
    let Ok(entries) = fs::read_dir(dir) else {
        return 0;
    };
    entries
        .flatten()
        .filter(|e| {
            e.path()
                .extension()
                .map_or(false, |ext| ext == "json")
        })
        .count()
}

fn parse_file_summary(path: &Path) -> Option<SessionSummary> {
    let content = fs::read_to_string(path).ok()?;
    let history: ChatHistory = serde_json::from_str(&content).ok()?;

    let session_title = history
        .data
        .biz_data
        .chat_session
        .title
        .unwrap_or_else(|| "未命名".to_string());

    let updated = format_timestamp(history.data.biz_data.chat_session.updated_at);

    let questions: Vec<String> = history
        .data
        .biz_data
        .chat_messages
        .iter()
        .filter(|m| m.role == "USER")
        .filter_map(|m| {
            m.fragments.iter().find(|f| f.frag_type == "REQUEST").map(|f| {
                f.content.trim().to_string()
            })
        })
        .collect();

    let question_count = questions.len();
    let total_chars = questions.iter().map(|q| q.len()).sum();
    let sample_questions: Vec<String> = questions
        .into_iter()
        .take(3)
        .map(|q| truncate(&q, 80))
        .collect();

    if question_count == 0 {
        return None;
    }

    Some(SessionSummary {
        title: session_title,
        updated_at: updated,
        question_count,
        sample_questions,
        total_chars,
    })
}

fn format_timestamp(ts: Option<f64>) -> String {
    match ts {
        Some(t) => {
            let secs = t as i64;
            let days_since_epoch = secs / 86400;
            let time_of_day = secs % 86400;
            let hours = time_of_day / 3600;
            let minutes = (time_of_day % 3600) / 60;
            format!("第{}天 {:02}:{:02}", days_since_epoch, hours, minutes)
        }
        None => "未知时间".to_string(),
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        let end = s
            .char_indices()
            .take(max_len)
            .last()
            .map(|(i, _)| i)
            .unwrap_or(max_len);
        format!("{}...", &s[..end])
    }
}