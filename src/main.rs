mod zhihu;
mod config;
use tokio;

#[tokio::main]
async fn main() {
    let urls = zhihu::init_urls();
    let body = zhihu::get_from_id(urls.get("question").unwrap()).await;
    let questions = zhihu::get_question_list(body);
    for [url, title] in questions {
        println!("{}: {}", url, title);
    }
}