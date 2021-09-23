use log::info;
use serde::{Deserialize, Serialize};

use crate::weather::WEATHERCLIENT;

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherDay {
    pub day: String,
    pub temperature: String,
    pub wind: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherResponse {
    pub temperature: String,
    pub wind: String,
    pub description: String,

    #[serde(skip)]
    pub forecast: Vec<WeatherDay>,
}

pub async fn request_weather(city: &str) -> Option<WeatherResponse> {
    let request = format!("https://goweather.herokuapp.com/weather/{}", city);

    let response = (*WEATHERCLIENT)
        .get(request)
        .header("cache-control", "no-cache")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    if let Ok(out) = serde_json::from_str::<WeatherResponse>(&response) {
        info!("Found weather information for {}", city);
        return Some(out);
    } else {
        info!("Unable to find weather information for {}", city);
        return None;
    }
}
