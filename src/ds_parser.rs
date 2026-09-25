use std::fs;
use std::path::Path;

use crate::types::{ChatHistory, UserQuestion};

pub fn parse_dir(dir: &str) -> Vec<UserQuestion> {
    let mut questions = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        eprintln!("无法读取目录: {}", dir);
        return questions;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(true, |e| e != "json") {
            continue;
        }
        if let Some(qs) = parse_file(&path) {
            questions.extend(qs);
        }
    }

    questions.sort_by_key(|q| q.message_id);
    questions
}

fn parse_file(path: &Path) -> Option<Vec<UserQuestion>> {
    let content = fs::read_to_string(path).ok()?;
    let history: ChatHistory = serde_json::from_str(&content).ok()?;
    let session_title = history
        .data
        .biz_data
        .chat_session
        .title
        .unwrap_or_else(|| "未命名".to_string());

    let questions: Vec<UserQuestion> = history
        .data
        .biz_data
        .chat_messages
        .iter()
        .filter(|m| m.role == "USER")
        .filter_map(|m| {
            m.fragments
                .iter()
                .find(|f| f.frag_type == "REQUEST")
                .map(|f| UserQuestion {
                    session_title: session_title.clone(),
                    question: f.content.trim().to_string(),
                    message_id: m.message_id,
                })
        })
        .collect();

    if questions.is_empty() {
        None
    } else {
        Some(questions)
    }
}

pub fn total_questions(questions: &[UserQuestion]) -> usize {
    questions.len()
}

pub fn total_sessions(questions: &[UserQuestion]) -> usize {
    let mut titles: Vec<&str> = questions.iter().map(|q| q.session_title.as_str()).collect();
    titles.sort_unstable();
    titles.dedup();
    titles.len()
}