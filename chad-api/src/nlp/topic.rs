use derive_more::Display;
use log::info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use strum::{EnumIter, IntoEnumIterator};

use crate::nlp::{NLPCLIENT, NLPTOKEN};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TopicOutput {
    pub labels: Vec<String>,
    pub scores: Vec<f64>,
}

#[derive(EnumIter, Debug, PartialEq, Clone, Copy, Display)]
pub enum Topic {
    News,
    Weather,
    Conversation,
}

static TOPICS_STRING: Lazy<Vec<String>> = Lazy::new(|| {
    Topic::iter()
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
});

static TOPICS: Lazy<Vec<Topic>> = Lazy::new(|| Topic::iter().collect::<Vec<Topic>>());

/// Determines the topic of the message sent by a user. Usually the first step in the reference API.
///
/// # Examples
/// ```
/// # use chad_api::nlp::topic::{determine_topic, Topic};
///
/// #[actix_web::main]
/// async fn main() {
///     assert_eq!(Topic::Weather, determine_topic("What's up with the weather?").await);
///     assert_eq!(Topic::News, determine_topic("What's up with the world?").await);
///     assert_eq!(Topic::Conversation, determine_topic("What's up?").await);
/// }
/// ```
pub async fn determine_topic(message: &str) -> Option<Topic> {
    let json_request = json!({
        "text": message,
        "labels": *TOPICS_STRING,
        "multi_class": true
    });

    info!("Sending request: {:?}", json_request);

    // Determine the topic using nlpcloud
    // - Forgetting $NLPTOKEN will panic on first evaluation due to NLPTOKEN being Lazy
    // - Will serialize the text output into a TopicOutput
    // TODO: Handle errors more gracefully instead of unwrapping
    let preresponse = (*NLPCLIENT)
        .post("https://api.nlpcloud.io/v1/bart-large-mnli/classification")
        .header("Authorization", format!("Token {}", *NLPTOKEN))
        .json(&json_request)
        .send()
        .await
        .unwrap()
        .json::<TopicOutput>()
        .await;

    let response = if let Ok(output) = preresponse {
        output
    } else {
        return None;
    };

    info!("Server returned {:?}", response);

    let response_labels = response.clone().labels;

    let unsorted_scores = response
        .scores
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, f64)>>();

    let mut top_score = f64::MIN;
    let mut top_topic = Topic::Conversation;

    for (this_score_id, this_score_value) in unsorted_scores {
        if this_score_value > top_score {
            top_score = this_score_value;
            top_topic = (*TOPICS)[(*TOPICS_STRING)
                .iter()
                .position(|s| s == &response_labels[this_score_id])
                .unwrap()];
        }
    }

    info!("Determined topic: {:?}", top_topic);

    return Some(top_topic);
}
