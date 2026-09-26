use crate::types::{SessionDetail, SessionSummary};

pub fn format_session_list(
    summaries: &[SessionSummary],
    total_count: usize,
    limit: usize,
) -> String {
    let mut output = String::new();
    output.push_str(&format!(
        "# 会话概览（共 {} 个会话，展示最近 {} 个）\n\n",
        total_count, limit
    ));
    output.push_str("## 全局统计\n\n");
    output.push_str(&format!("- 目录中总会话数: {}\n", total_count));
    output.push_str(&format!(
        "- 已加载摘要: {} 个\n",
        summaries.len()
    ));

    let total_q: usize = summaries.iter().map(|s| s.question_count).sum();
    output.push_str(&format!("- 已加载会话总提问数: {}\n", total_q));

    let total_chars: usize = summaries.iter().map(|s| s.total_chars).sum();
    output.push_str(&format!(
        "- 已加载会话提问总字数: {} (~{:.0}KB)\n\n",
        total_chars,
        total_chars as f64 / 1000.0
    ));

    output.push_str("## 会话列表\n\n");

    for (i, s) in summaries.iter().enumerate() {
        output.push_str(&format!("### {}. {}\n", i + 1, s.title));
        output.push_str(&format!("- 时间: {}\n", s.updated_at));
        output.push_str(&format!("- 提问数: {}\n", s.question_count));
        output.push_str("- 提问示例:\n");
        for q in &s.sample_questions {
            output.push_str(&format!("  - \"{}\"\n", q));
        }
        output.push('\n');
    }

    if total_count > limit {
        output.push_str(&format!(
            "⚠ 仅展示了最近 {} / {} 个会话。如需查看更多，请用 `zhihu_session` 按标题关键词搜索指定会话的详情。\n",
            summaries.len(),
            total_count
        ));
    }

    output
}

pub fn format_session_detail(detail: &SessionDetail) -> String {
    let mut output = String::new();
    output.push_str(&format!("# {}\n\n", detail.title));
    output.push_str(&format!("更新时间: {}\n\n", detail.updated_at));
    output.push_str("---\n\n");

    for turn in &detail.turns {
        if turn.role == "USER" {
            output.push_str(&format!("### ❓ 用户提问\n{}\n\n", turn.content));
        } else {
            output.push_str(&format!("### 🤖 AI 回复\n{}\n\n", turn.content));
        }
    }

    output.push_str(&format!(
        "\n---\n*共 {} 轮对话，内容已截断至每段 500 字*\n",
        detail.turns.len()
    ));

    output
}

pub fn format_zhihu_for_ai(
    zhihu_questions: &[[String; 2]],
    summaries: &[SessionSummary],
    total_count: usize,
    limit: usize,
) -> String {
    let mut output = String::new();

    output.push_str("# 知乎日常 — 原始数据\n\n");

    output.push_str("## 用户会话上下文\n\n");
    output.push_str("以下是你最近和最频繁的 DS 对话主题摘要，请在匹配时参考这些上下文:\n\n");
    for (i, s) in summaries.iter().enumerate().take(10) {
        output.push_str(&format!(
            "{}. **{}** ({} 条提问)\n",
            i + 1,
            s.title,
            s.question_count
        ));
        for q in &s.sample_questions {
            output.push_str(&format!("   - \"{}\"\n", q));
        }
    }
    output.push('\n');

    output.push_str("## 知乎推荐问题\n\n");
    output.push_str(&format!(
        "共拉取 {} 个推荐问题（关联度需由你结合上下文判断）:\n\n",
        zhihu_questions.len()
    ));

    for [id, title] in zhihu_questions {
        output.push_str(&format!("- `{}` | **{}**\n", id, title));
    }

    output.push_str("\n---\n");
    output.push_str("## 给你的指令\n\n");
    output.push_str("请根据以下 SKILL.md 规则处理上述数据:\n\n");
    output.push_str("1. 分析会话摘要，识别用户的**5-8 个核心兴趣领域**\n");
    output.push_str("2. 从知乎推荐中挑出与兴趣相关的 TOP 3 问题\n");
    output.push_str("3. 对每个匹配问题生成 **100 字 Markdown 回答**\n");
    output.push_str("4. 回答风格: 技术类偏工程实践、社会人文类点明矛盾、消费类以实际体验为依据\n");
    output.push_str("5. 从剩余问题中选一条用户可能会问的作为 **今日提问**\n");
    output.push_str("6. 格式: ## 今日回答 / Q: ... / A: ... / ## 今日提问\n");
    output.push_str(&format!(
        "\n*会话共 {} 个（本次加载 {} 个），如需某个会话的完整详情请调用 `zhihu_session`。*\n",
        total_count, limit
    ));

    output
}