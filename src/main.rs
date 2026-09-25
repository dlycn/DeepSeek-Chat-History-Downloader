mod analyzer;
mod config;
mod ds_parser;
mod settings;
mod types;
mod zhihu;

use clap::{Parser, Subcommand};
use settings::Settings;

#[derive(Parser)]
#[command(name = "zhihu-daily", about = "知乎日常助手: 解析DS对话, 匹配知乎问题, 生成回答建议")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init,
    Parse {
        #[arg(short, long)]
        dir: Option<String>,
    },
    Daily {
        #[arg(short, long)]
        dir: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => cmd_init(),
        Command::Parse { dir } => {
            let settings = settings::load_or_init();
            let d = dir.as_deref().unwrap_or(&settings.deepseek_dir);
            cmd_parse(d);
        }
        Command::Daily { dir } => {
            let settings = settings::load_or_init();
            let d = dir.as_deref().unwrap_or(&settings.deepseek_dir);
            cmd_daily(d, &settings).await;
        }
    }
}

fn cmd_init() {
    settings::run_init();
}

fn cmd_parse(dir: &str) {
    let questions = ds_parser::parse_dir(dir);
    let total_q = ds_parser::total_questions(&questions);
    let total_s = ds_parser::total_sessions(&questions);
    let profile = analyzer::build_profile(&questions);

    println!("# DeepSeek 对话分析报告\n");
    println!("**会话数**: {}", total_s);
    println!("**提问数**: {}\n", total_q);
    println!("## 兴趣领域分布\n");
    for (i, topic) in profile.topics.iter().enumerate() {
        let sample = profile
            .sample_questions
            .get(i)
            .map(|s| s.as_str())
            .unwrap_or("");
        println!("- **{}**: {}", topic, sample);
    }
    println!("\n## 提问特征\n");
    println!("- 平均提问长度: {:.0} 字", profile.avg_question_len);

    println!("\n## 可提取的知乎候选问题\n");
    for q in &questions {
        let short = if q.question.len() > 60 {
            format!("{}...", &q.question[..57])
        } else {
            q.question.clone()
        };
        println!("- [{}] {}", q.session_title, short);
    }
}

async fn cmd_daily(dir: &str, settings: &Settings) {
    let questions = ds_parser::parse_dir(dir);
    let profile = analyzer::build_profile(&questions);

    println!("# 知乎日常任务 ({})\n", chrono_or_naive());

    println!("## 用户画像\n");
    println!("- 兴趣领域: {}", profile.topics.join(", "));
    println!(
        "- 基于 {} 个会话, {} 条提问分析\n",
        ds_parser::total_sessions(&questions),
        profile.question_count
    );

    match (&settings.zhihu_cookie, &settings.zhihu_xsrf) {
        (Some(cookie), Some(xsrf)) => {
            let headers = config::build_headers(cookie, xsrf);
            let urls = zhihu::init_urls();
            let body = zhihu::get_from_id(urls.get("question").unwrap(), headers).await;
            let zhihu_questions = zhihu::get_question_list(body);

            let tasks = analyzer::match_with_zhihu(&profile, &zhihu_questions);

            println!("## 匹配的知乎问题\n");
            if tasks.is_empty() {
                println!("暂无匹配的问题, 以下是全部知乎推荐:\n");
                for [id, title] in &zhihu_questions {
                    println!("- `{}`: {}", id, title);
                }
            } else {
                for task in &tasks {
                    println!("### {}", task.zhihu_title);
                    println!("- ID: `{}`", task.zhihu_question_id);
                    println!("- 关联度: {}", task.relevance);
                    println!("- 建议角度: {}", task.suggested_angle);
                    println!("- 用户相关话题: {}\n", task.related_ds_topics.join(", "));
                }
            }
        }
        _ => {
            println!("## 知乎问题\n");
            println!("未配置知乎凭据，无法拉取推荐问题。");
            println!("请运行 `init` 命令配置 Cookie 后重试。\n");
            println!("以下是基于 DS 对话的候选问题供参考:\n");
            for q in &questions {
                let short = if q.question.len() > 80 {
                    format!("{}...", &q.question[..77])
                } else {
                    q.question.clone()
                };
                println!("- [{}] {}", q.session_title, short);
            }
        }
    }

    println!("\n---");
    println!("*请 AI 助手根据上述任务列表, 对匹配的问题逐个生成 100 字以内的 Markdown 回答。*");
}

fn chrono_or_naive() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let days_since_epoch = secs / 86400;
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    format!("第{}天 {:02}:{:02}", days_since_epoch, hours, minutes)
}