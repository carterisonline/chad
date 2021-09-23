use async_trait::async_trait;
use chad_api::news::request::NewsResponse;
use chad_api::nlp::topic::Topic;
use chad_api::weather::request::WeatherResponse;
use chad_api::{PlainResponse, PORT};
use serde::Serialize;
use serenity::utils::MessageBuilder;

use crate::Handler;

macro_rules! return_bot_error {
    () => {
        return format!("Oops! I ran into an error on the bot-level. Please contact Carter!")
    };

    ($msg: expr) => {
        println!("{}", $msg);
        return format!("Oops! I ran into an error on the bot-level. Please contact Carter!")
    };

    ($msg: expr, $debug_data: expr) => {
        println!("{}{:?}", $msg, $debug_data);
        return format!("Oops! I ran into an error on the bot-level. Please contact Carter!")
    };
}

#[async_trait]
pub trait ParseMessage {
    async fn parse_message(&self, msg: &str) -> String;
}

#[async_trait]
impl ParseMessage for Handler {
    async fn parse_message(&self, msg: &str) -> String {
        let response = self
            .api_client
            .get(format!("http://{}:{}", self.api_addr, PORT))
            .header("message", msg)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        if let Some(captures) = self.type_regex.captures(&response) {
            if let (Some(response_type), Some(body)) = (captures.get(1), captures.get(2)) {
                match (response_type.as_str(), body.as_str()) {
                    ("PlainResponse", body) => {
                        if let Ok(parsed) = json5::from_str::<PlainResponse>(body) {
                            return parsed.message;
                        } else {
                            return_bot_error!("Unable to parse PlainResponse from ", body);
                        }
                    }

                    ("NewsResponse", body) => {
                        if let Ok(parsed) = json5::from_str::<NewsResponse>(body) {
                            MessageBuilder::new().push(parsed.url).build()
                        } else {
                            return_bot_error!("Unable to parse NewsResponse from ", body);
                        }
                    }

                    ("WeatherResponse", body) => {
                        if let Ok(parsed) = json5::from_str::<WeatherResponse>(body) {
                            MessageBuilder::new()
                                .push_bold_line_safe("-- Current Weather --")
                                .push_line_safe(format!("Temperature: {}", parsed.temperature))
                                .push_line_safe(format!("Wind: {}", parsed.wind))
                                .push_line_safe(parsed.description)
                                .build()
                        } else {
                            return_bot_error!("Unable to parse WeatherResponse from ", body);
                        }
                    }

                    _ => {
                        println!("Response: {}", response);
                        return_bot_error!("Uncaught handler for type ", response_type);
                    }
                }
            } else {
                return_bot_error!("Unable to get captures for ", captures);
            }
        } else {
            return_bot_error!("Unable to complete the capture operation for ", response);
        }
    }
}
