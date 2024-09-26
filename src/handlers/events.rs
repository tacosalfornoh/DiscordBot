use serenity::all::{Command, Context, CreateInteractionResponse, CreateInteractionResponseMessage, CurrentUser, EventHandler, Guild, GuildId, Interaction, Member, Message, PartialGuild, Permissions, Ready, UnavailableGuild};
use serenity::async_trait;
use crate::{commands};
use crate::handlers::database::{db_delete_guild, db_insert_guild, db_insert_member, db_update_guild};
use crate::components::level::add_experience;

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, _ctx: Context, guild: Guild, is_new: Option<bool>) {
        db_insert_guild(guild.id.into(), &guild.name).await.expect("TODO: panic message");
        println!(
            "Guild {} ({}) created by {} has been {}",
            guild.name,
            guild.id,
            guild.owner_id,
            if is_new.unwrap_or(false) { "created" } else { "joined" },
        );
    }

    async fn guild_delete(&self, _ctx: Context, incomplete: UnavailableGuild, full: Option<Guild>) {
        println!(
            "Guild {} ({}) has been {}",
            incomplete.unavailable,
            incomplete.id,
            if full.is_none() { "deleted" } else { "left" }
        );
        db_delete_guild(incomplete.id.into()).await.expect("TODO: panic message");
    }

    async fn guild_member_addition(&self, _ctx: Context, new_member: Member) {
        println!("User {} has joined the guild at {:?}", new_member.user.name, new_member.joined_at);
        db_insert_member(new_member.user.id.into(), &new_member.user.name, new_member.joined_at.unwrap().to_string()).await.expect("TODO: panic message");
    }

    async fn guild_update(&self, _ctx: Context, _old: Option<Guild>, _new: PartialGuild) {
        println!("Guild {} ({}) has been updated", _new.name, _new.id);
        db_update_guild(_new.id.into(), &_new.name).await.expect("TODO: panic message");
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        match msg.content.as_str() {
            "!ping" => {
                if let Err(why) = msg.channel_id.say(&_ctx.http, "Pong!").await {
                    println!("Error sending message: {why:?}");
                }
            }
            _ => {
                add_experience(msg.author.id.into()).await;
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        // TODO: Registering slash commands for a guild (discord)
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
        // TODO: Registering global slash commands
        let guild_command = Command::set_global_commands(
            &ctx.http,
            vec![
                commands::admin::application::register(),
                commands::admin::level::register(),
                commands::admin::balance::register(),
                commands::misc::ping::register(),
            ],
        )
            .await;
        println!("I created the following global slash command: {guild_command:#?}");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => Some(commands::misc::ping::run(&command.data.options())),
                "application" => {
                    // TODO: Check if the user has ADMINISTRATOR permissions
                    match command.member.as_ref().map_or(false, |m| {
                        m.permissions.unwrap().contains(Permissions::ADMINISTRATOR)
                    }) {
                        true => Some(commands::admin::application::run(&command.data.options())),
                        false => {
                            Some("You do not have permission to use this command.".to_string())
                        }
                    }
                }
                "level" => {
                    // TODO: Check if the user has ADMINISTRATOR permissions
                    match command.member.as_ref().map_or(false, |m| {
                        m.permissions.unwrap().contains(Permissions::ADMINISTRATOR)
                    }) {
                        true => Some(commands::admin::level::run(&command.data.options())),
                        false => {
                            Some("You do not have permission to use this command.".to_string())
                        }
                    }
                }
                "balance" => {
                    // TODO: Check if the user has ADMINISTRATOR permissions
                    match command.member.as_ref().map_or(false, |m| {
                        m.permissions.unwrap().contains(Permissions::ADMINISTRATOR)
                    }) {
                        true => Some(commands::admin::balance::run(&command.data.options())),
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