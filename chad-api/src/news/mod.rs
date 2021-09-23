use std::env;

use once_cell::sync::Lazy;
use reqwest::Client;

pub mod date_format;
pub mod request;

pub(self) static CURRENTSTOKEN: Lazy<String> =
    Lazy::new(|| env::var("CURRENTSTOKEN").expect("$CURRENTSTOKEN not provided!"));

pub(self) static NEWSCLIENT: Lazy<Client> = Lazy::new(|| Client::new());
