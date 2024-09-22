mod api;
mod commands;
mod templates;
use templates::configuration::create_guild;

use mongodb::{
    bson:: Document,
    Client, Collection,
};
use serenity::all::{Guild, PartialGuild, Permissions, UnavailableGuild};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // ? Registering slash commands for a guild (discord)
        let guild_id = GuildId::new(
            "940226887463100416" // * 940226887463100416 is the ID of the guild I'm in
                .parse()
                .expect("Failed to parse guild ID"),
        );
        let commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::misc::ping::register()
                ])
            .await;
        println!("I now have the following guild slash commands: {commands:#?}");

        // ? Registering global slash commands
        let guild_command = Command::set_global_commands(
            &ctx.http,
            vec![
                commands::admin::application::register(),
            ],
        )
        .await;
        println!("I created the following global slash command: {guild_command:#?}");
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild, is_new: Option<bool>) {
        // TODO: Save the guild to the database
        println!(
            "Guild {} ({}) has been {}",
            guild.name,
            guild.id,
            if is_new.unwrap_or(false) { "created" } else { "joined" }
        );
    }
    async fn guild_update(&self, _ctx: Context, _old: Option<Guild>, _new: PartialGuild) {
        // TODO: Update the guild in the database
        println!("Guild {} ({}) has been updated", _new.name, _new.id);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => Some(commands::misc::ping::run(&command.data.options())),
                "application" => {
                    // ? Check if the user has ADMINISTRATOR permissions
                    match command.member.as_ref().map_or(false, |m| {
                        m.permissions.unwrap().contains(Permissions::ADMINISTRATOR)
                    }) {
                        true => Some(commands::admin::application::run(&command.data.options())),
                        false => {
                            Some("You do not have permission to use this command.".to_string())
                        }
                    }
                }
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}

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

    let token = "";
    let mut client = serenity::Client::builder(token, GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILDS)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
