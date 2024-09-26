mod api;
mod commands;
mod templates;
mod handlers;
mod custom_fn;
mod components;

use serenity::prelude::*;
use serenity::all::GatewayIntents;
use crate::handlers::events::Handler;

const TOKEN: &str = "MTI0ODMwODY5NTMyMzExNTU0Mw.GJYA4t.vrEGZXQOYDk1KBZa8egRSEXDvHbRgiUzAr0QtA";
#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut client = Client::builder(TOKEN, GatewayIntents::GUILD_MEMBERS |
        GatewayIntents::GUILDS |
        GatewayIntents::GUILD_MESSAGES |
        GatewayIntents::GUILD_MESSAGE_REACTIONS |
        GatewayIntents::GUILD_VOICE_STATES |
        GatewayIntents::GUILD_PRESENCES |
        GatewayIntents::GUILD_INVITES |
        GatewayIntents::GUILD_EMOJIS_AND_STICKERS |
        GatewayIntents::GUILD_MESSAGE_TYPING |
        GatewayIntents::MESSAGE_CONTENT |
        GatewayIntents::GUILD_INTEGRATIONS)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}