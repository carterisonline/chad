use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use chad_api::news::request::request_article;
use chad_api::nlp::question_answering::answer_question;
use chad_api::nlp::topic::{determine_topic, Topic};
use chad_api::text::{get_text, get_text_lonely};
use chad_api::weather::request::request_weather;
use chad_api::{plain, PORT};

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    if let Some(message) = req.headers().get("message") {
        if let Ok(output) = message.to_str() {
            match determine_topic(output).await {
                Some(Topic::News) => {
                    if let Some(article) =
                        answer_question(output, "What news topic does the message ask for?").await
                    {
                        if let Some(article_link) = request_article(&article).await {
                            return format!("{:?}", article_link);
                        } else {
                            return plain!(format!(
                                "{} \"{}.\" {}",
                                get_text("no-article"),
                                article,
                                get_text("try-different-keywords")
                            ));
                        }
                    } else {
                        return get_text_lonely("rate-limited");
                    }
                }
                Some(Topic::Weather) => {
                    if let Some(city) =
                        answer_question(output, "What city is the message asking for weather for?")
                            .await
                    {
                        if let Some(weather_data) = request_weather(&city).await {
                            return format!("{:?}", weather_data);
                        } else {
                            return format!("{} {}.", get_text("no-weather"), city);
                        }
                    } else {
                        return get_text_lonely("rate-limited");
                    }
                }
                Some(topic) => return plain!(topic),
                None => return get_text_lonely("rate-limited"),
            }
        } else {
            return get_text_lonely("invalid-message");
        }
    } else {
        return get_text_lonely("no-message");
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| App::new().service(index))
        .bind(format!("127.0.0.1:{}", PORT))?
        .run()
        .await
}
