use once_cell::sync::Lazy;
use reqwest::Client;

pub mod request;

pub(self) static WEATHERCLIENT: Lazy<Client> = Lazy::new(|| Client::new());
