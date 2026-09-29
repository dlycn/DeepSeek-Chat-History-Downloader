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