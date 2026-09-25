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

#[derive(Debug)]
pub struct UserQuestion {
    pub session_title: String,
    pub question: String,
    pub message_id: i64,
}

#[derive(Debug)]
pub struct TopicProfile {
    pub topics: Vec<String>,
    pub question_count: usize,
    pub avg_question_len: f64,
    pub sample_questions: Vec<String>,
}

#[derive(Debug)]
pub struct DailyTask {
    pub zhihu_question_id: String,
    pub zhihu_title: String,
    pub relevance: String,
    pub related_ds_topics: Vec<String>,
    pub suggested_angle: String,
}