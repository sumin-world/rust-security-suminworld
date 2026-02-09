mod handler;
mod helpers;
mod models;
mod scanner;

use handler::Handler;
use models::AppState;
use once_cell::sync::Lazy;
use serenity::prelude::*;
use std::env;
use tokio::sync::RwLock;

pub(crate) static STATE: Lazy<RwLock<AppState>> = Lazy::new(|| RwLock::new(AppState::from_env()));

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    {
        let mut st = STATE.write().await;
        *st = AppState::from_env();
    }

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_PRESENCES
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
    }
}
