use std::env;

use once_cell::sync::Lazy;
use reqwest::Client;

pub mod question_answering;
pub mod topic;

pub(self) static NLPTOKEN: Lazy<String> =
    Lazy::new(|| env::var("NLPTOKEN").expect("$NLPTOKEN not provided!"));

pub(self) static NLPCLIENT: Lazy<Client> = Lazy::new(|| Client::new());
