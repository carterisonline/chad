use anyhow::{Context, Error, Result};
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::news::{CURRENTSTOKEN, NEWSCLIENT};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewsResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub author: String,
    pub image: String,
    pub language: String,
    pub category: Vec<String>,

    #[serde(skip)]
    pub published: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurrentsResponse {
    status: String,
    pub news: Vec<NewsResponse>,
}

pub async fn request_article(keyword: &str) -> Option<NewsResponse> {
    let request = format!(
        "https://api.currentsapi.services/v1/search?keywords={}&language={}",
        keyword, "en"
    );

    let response = (*NEWSCLIENT)
        .get(request)
        .header("Authorization", (*CURRENTSTOKEN).clone())
        .header("cache-control", "no-cache")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    if let Ok(out) = serde_json::from_str::<CurrentsResponse>(&response) {
        info!("Was able to find an article about {}", keyword);
        let news = out.news;

        match news.get(0) {
            Some(id) => return Some(id.clone()),
            None => {
                error!("Response error! Got {:?}", response);
                return None;
            }
        }
    } else {
        error!("Response error! Got {:?}", response);
        return None;
    }
}
