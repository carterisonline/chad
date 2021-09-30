use anyhow::Result;
use derive_more::Display;
use log::info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use strum::{EnumIter, IntoEnumIterator};

use crate::classification;
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
pub async fn determine_topic(message: &str) -> Result<Topic> {
    let asref: Vec<&str> = (*TOPICS_STRING).iter().map(String::as_str).collect();

    let topic = classification::classify(message, &asref[..])?;

    info!("Determined topic: {:?}", topic);

    let topic_enum = (*TOPICS)[(*TOPICS_STRING)
        .iter()
        .position(|s| s == &topic.as_str())
        .unwrap()];

    return Ok(topic_enum);
}
