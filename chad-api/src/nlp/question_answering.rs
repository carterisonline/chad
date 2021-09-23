use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::nlp::{NLPCLIENT, NLPTOKEN};

#[derive(Serialize, Deserialize)]
struct AnswerQuestionResponse {
    answer: String,
    score: f64,
    start: usize,
    end: usize,
}

pub async fn answer_question(context: &str, question: &str) -> Option<String> {
    let json_request = json!({
        "context": context,
        "question": question,
    });

    let response = (*NLPCLIENT)
        .post("https://api.nlpcloud.io/v1/roberta-base-squad2/question")
        .header("Authorization", format!("Token {}", *NLPTOKEN))
        .json(&json_request)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    if let Ok(out) = serde_json::from_str::<AnswerQuestionResponse>(&response) {
        info!(
            "Our questions have been answered! \"{}\" -> {}",
            question, out.answer
        );
        return Some(out.answer);
    } else {
        return None;
    }
}
