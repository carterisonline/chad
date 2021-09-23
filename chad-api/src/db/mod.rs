use std::env;

use once_cell::sync::Lazy;
use redis::Client;

pub(self) static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::open(format!(
        "redis://{}",
        env::var("REDIS_ADDR").expect("$REDIS_ADDR not provided!")
    ))
    .expect("Failed to connect to Redis client. Is `redis-server` running?")
});
