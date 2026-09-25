use std::collections::HashMap;

use crate::types::{DailyTask, TopicProfile, UserQuestion};

const TOPIC_KEYWORDS: &[(&str, &[&str])] = &[
    ("Rust编程", &["rust", "借用", "所有权", "trait", "枚举", "异步", "tokio", "cargo", "生命周期", "宏", "move", "引用"]),
    ("游戏开发", &["bevy", "游戏", "渲染", "物理", "碰撞", "2d", "3d", "动画", "着色器", "avian"]),
    ("前端/GUI", &["tauri", "前端", "gui", "web", "html", "css", "ui", "界面", "窗口", "组件"]),
    ("工具链/DevOps", &["git", "ssh", "sql", "数据库", "命令行", "部署", "docker", "编译", "构建"]),
    ("社会人文", &["矛盾", "社会", "政治", "家庭", "婆媳", "关系", "时代", "平等", "权利", "女性"]),
    ("数学/科学", &["数学", "公式", "证明", "物理", "科学", "模拟", "计算", "几何", "函数"]),
    ("生活消费", &["推荐", "便宜", "好用", "购买", "山姆", "超市", "日常", "穿", "穿穿"]),
    ("AI/机器学习", &["ai", "智能", "tensorboard", "训练", "模型", "深度学习", "神经网络", "机器学习"]),
    ("哲学/思辨", &["哲学", "存在", "意识", "世界", "唯心", "唯物", "假说", "感知"]),
    ("Android/移动", &["android", "蓝牙", "手机", "app", "音量", "安卓"]),
];

pub fn build_profile(questions: &[UserQuestion]) -> TopicProfile {
    let mut topic_counts: HashMap<&str, usize> = HashMap::new();
    let mut topic_samples: HashMap<&str, Vec<String>> = HashMap::new();
    let mut total_len: f64 = 0.0;

    for q in questions {
        total_len += q.question.len() as f64;
        let lower = q.question.to_lowercase();

        for (topic, keywords) in TOPIC_KEYWORDS {
            if keywords.iter().any(|kw| lower.contains(kw)) {
                *topic_counts.entry(topic).or_insert(0) += 1;
                topic_samples
                    .entry(topic)
                    .or_default()
                    .push(q.question.clone());
            }
        }
    }

    let mut sorted: Vec<(&str, usize)> = topic_counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let topics: Vec<String> = sorted.iter().map(|(t, _)| t.to_string()).collect();
    let sample_questions: Vec<String> = sorted
        .iter()
        .filter_map(|(t, _)| {
            topic_samples
                .get(t)
                .and_then(|s| s.first().cloned())
        })
        .collect();

    TopicProfile {
        topics,
        question_count: questions.len(),
        avg_question_len: if questions.is_empty() {
            0.0
        } else {
            total_len / questions.len() as f64
        },
        sample_questions,
    }
}

pub fn match_with_zhihu(
    profile: &TopicProfile,
    zhihu_questions: &[[String; 2]],
) -> Vec<DailyTask> {
    zhihu_questions
        .iter()
        .filter_map(|q| {
            let title_lower = q[1].to_lowercase();
            let mut matched_topics: Vec<String> = Vec::new();

            for topic in &profile.topics {
                for (t, keywords) in TOPIC_KEYWORDS {
                    if *t == topic.as_str()
                        && keywords
                            .iter()
                            .any(|kw| title_lower.contains(kw))
                    {
                        matched_topics.push(topic.clone());
                        break;
                    }
                }
            }

            if matched_topics.is_empty() {
                return None;
            }

            let relevance = format!(
                "匹配用户兴趣领域: {}",
                matched_topics.join(", ")
            );

            let suggested_angle = match matched_topics.first() {
                Some(t) if t == "Rust编程" => "从工程实践角度切入".to_string(),
                Some(t) if t == "游戏开发" => "从开发者经验角度分享".to_string(),
                Some(t) if t == "社会人文" => "结合社会观察展开分析".to_string(),
                Some(t) if t == "生活消费" => "以实际体验给出建议".to_string(),
                Some(t) if t == "工具链/DevOps" => "从效率提升角度说明".to_string(),
                _ => "从个人经验出发回答".to_string(),
            };

            Some(DailyTask {
                zhihu_question_id: q[0].clone(),
                zhihu_title: q[1].clone(),
                relevance,
                related_ds_topics: matched_topics,
                suggested_angle,
            })
        })
        .collect()
}