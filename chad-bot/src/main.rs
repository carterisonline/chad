#![feature(format_args_capture)]
#![feature(async_closure)]

use std::{env, process};

use commands::message::ParseMessage;
use log::info;
use regex::Regex;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::event::ResumedEvent;
use serenity::model::prelude::Ready;
use serenity::utils::MessageBuilder;
use serenity::{async_trait, Client};

mod commands;

struct Handler {
    api_client: reqwest::Client,
    api_addr: String,
    type_regex: Regex,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.mentions_me(&ctx.http).await.unwrap() {
            msg.react(&ctx.http, '\u{1F6F0}').await.unwrap();
            if let Err(reason) = msg
                .channel_id
                .say(
                    &ctx.http,
                    self.parse_message(&msg.content_safe(ctx.cache).await).await,
                )
                .await
            {
                println!("Error sending message: {:?}", reason);
            }
        }
    }
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("{} is online!", ready.user.name);
    }

    async fn resume(&self, _ctx: Context, _event: ResumedEvent) {
        info!("Resumed after inactivity.");
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Please provide $DISCORD_TOKEN!");

    let mut client = Client::builder(&token)
        .event_handler(Handler {
            api_client: reqwest::Client::new(),
            api_addr: env::var("CHAD_API_ADDR").expect("Please provide $CHAD_API_ADDR!"),
            type_regex: Regex::new(r"(\w+)\s+(.+)").unwrap(),
        })
        .await
        .expect("Failed to create the main client.");

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        process::exit(0);
    });

    if let Err(reason) = client.start().await {
        println!("Client error: {:?}", reason);
    }
}
