use std::collections::HashMap;
use reqwest;
use reqwest::header::HeaderMap;
use serde_json::Value;

pub fn init_urls() -> HashMap<&'static str, &'static str> {
    let mut urls = HashMap::new();
    urls.insert("home", "https://www.zhihu.com/");
    urls.insert(
        "voter",
        "https://www.zhihu.com/api/v4/articles/2085374500649096302/voters",
    );
    urls.insert(
        "question",
        "https://www.zhihu.com/api/v4/creators/question_route/author_related/recommend?limit=200&offset=0&page_source=web_author_recommend&recom_domain_score_ab=1",
    );
    urls.insert(
        "answer",
        "https://www.zhihu.com/creator/featured-question/invited",
    );
    urls.insert("write", "https://zhuanlan.zhihu.com/write");
    urls
}

pub async fn get_from_id(url: &str, headers: HeaderMap) -> String {
    let client = reqwest::Client::new();
    match client.get(url).headers(headers).send().await {
        Ok(resp) => resp.text().await.unwrap_or_default(),
        Err(_) => String::new(),
    }
}

pub fn get_question_list(base: String) -> Vec<[String;2]> {
    let mut article_list = Vec::new();
    let json: Value = match serde_json::from_str(&base) {
        Ok(v) => v,
        Err(e) => {
            let snippet: String = base.chars().take(200).collect();
            article_list.push(["ERR".to_string(), format!("JSON解析失败: {}. 前200字: {}", e, snippet)]);
            return article_list;
        }
    };
    if let Some(data) = json["data"].as_array() {
        for item in data {
            if let (Some(url), Some(title)) =
                (item["question"]["id"].as_str(), item["question"]["title"].as_str())
            {
                article_list.push([url.to_string(), title.to_string()]);
            }
        }
    }
    if article_list.is_empty() {
        article_list.push(["ERR".to_string(), "知乎API返回数据为空或结构异常".to_string()]);
    }
    article_list
}