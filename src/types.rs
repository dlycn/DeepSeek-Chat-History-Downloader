#[derive(Debug, serde::Deserialize)]
pub struct ChatHistory {
    pub data: ChatData,
}

#[derive(Debug, serde::Deserialize)]
pub struct ChatData {
    pub biz_data: BizData,
}

#[derive(Debug, serde::Deserialize)]
pub struct BizData {
    pub chat_session: ChatSession,
    pub chat_messages: Vec<ChatMessage>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub title: Option<String>,
    pub updated_at: Option<f64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ChatMessage {
    pub message_id: i64,
    pub parent_id: Option<i64>,
    pub role: String,
    pub fragments: Vec<Fragment>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Fragment {
    #[serde(rename = "type")]
    pub frag_type: String,
    pub content: String,
}

pub struct SessionSummary {
    pub title: String,
    pub updated_at: String,
    pub question_count: usize,
    pub sample_questions: Vec<String>,
    pub total_chars: usize,
}

pub struct SessionDetail {
    pub title: String,
    pub updated_at: String,
    pub turns: Vec<ConversationTurn>,
}

pub struct ConversationTurn {
    pub role: String,
    pub content: String,
}