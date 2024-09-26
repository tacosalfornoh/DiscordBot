mod api;
mod commands;
mod templates;

mod handlers;

use templates::configuration::create_guild;

use mongodb::{
    bson::Document,
    Client, Collection,
};
use serenity::all::{Guild, PartialGuild, Permissions, Role, GatewayIntents, GuildChannel};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::{GuildId, RoleId};
use serenity::prelude::*;
use crate::handlers::events::Handler;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let uri =
        "mongodb+srv://root:qq68T*LLkxXvXCQ@albionguildmanagerdsbot.gh6amky.mongodb.net/?retryWrites=true&w=majority&appName=AlbionGuildManagerDSBot";
    let mongo = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to the server");
    let database = mongo.database("test");
    let _my_coll: Collection<Document> = database.collection("configurations");
    let my_doc = create_guild("Albion ELITE", "940226887463100415");
    println!("Found a movie:\n{:#?}", my_doc);

    let token = "MTI0ODMwODY5NTMyMzExNTU0Mw.GyPtbW.F4v0iwG4kRaeHemFPnf7xEMWc8bTpnjvA4cZnM";
    let mut client = serenity::Client::builder(token, GatewayIntents::GUILD_MEMBERS |
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
