pub mod db;
pub mod news;
pub mod nlp;
pub mod text;
pub mod weather;
pub mod wiki_qa;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub const PORT: &str = "4096";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PlainResponse {
    pub message: String,
}

impl Display for PlainResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PlainResponse {
    pub fn new<S: ToString>(message: S) -> Self {
        return PlainResponse {
            message: message.to_string(),
        };
    }
}

#[macro_export]
macro_rules! plain {
    () => {
        $crate::PlainResponse::default().to_string()
    };

    ($body: expr) => {
        $crate::PlainResponse::new($body).to_string()
    };
}
