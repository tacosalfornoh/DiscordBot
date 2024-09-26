use serenity::all::{Command, Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler, Guild, GuildChannel, GuildId, Interaction, Message, PartialGuild, Permissions, Ready, Role, RoleId, TypingStartEvent};
use serenity::async_trait;
use crate::{commands};

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn channel_create(&self, _ctx: Context, _channel: GuildChannel) {
        println!("Channel {} ({}) has been created", _channel.name, _channel.id);
        // TODO: Implement the channel_create event
    }
    async fn channel_update(&self, _ctx: Context, _old: Option<GuildChannel>, new: GuildChannel) {
        println!("Channel {} ({}) has been updated", new.name, new.id);
        // TODO: Implement the channel_update event
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild, is_new: Option<bool>) {
        println!(
            "Guild {} ({}) has been {}",
            guild.name,
            guild.id,
            if is_new.unwrap_or(false) { "created" } else { "joined" }
        );
        // TODO: Implement the guild_create event
    }

    async fn guild_role_create(&self, _ctx: Context, new: Role) {
        println!("Role {} ({}) has been created", new.name, new.id);
        // TODO: Implement the guild_role_create event
    }

    async fn guild_role_delete(&self, _ctx: Context, _guild: GuildId, role: RoleId, _guild_id: Option<Role>) {
        println!("Role {} ({}) has been deleted", role, _guild);
        // TODO: Implement the guild_role_delete event
    }

    async fn guild_role_update(&self, _ctx: Context, _old_data_if_available: Option<Role>, new: Role) {
        println!("Role {} ({}) has been updated", new.name, new.id);
        // TODO: Implement the guild_role_update event
    }

    async fn guild_update(&self, _ctx: Context, _old: Option<Guild>, _new: PartialGuild) {
        println!("Guild {} ({}) has been updated", _new.name, _new.id);
        // TODO: Implement the guild_update event
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        println!("{:#?}", msg);
        match msg.content.as_str() {
            "!ping" => {
                if let Err(why) = msg.channel_id.say(&_ctx.http, "Pong!").await {
                    println!("Error sending message: {why:?}");
                }
            }
            "ciao" => {
                msg.channel_id.broadcast_typing(&_ctx.http).await.expect("TODO: panic message");
                //write into the channel

                if let Err(why) = msg.channel_id.say(&_ctx.http, "Ciao!").await {
                    println!("Error sending message: {why:?}");
                }
            }
            _ => {
                println!("Received a message: {}", msg.content);
            }
        }
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an authentication error, or lack
            // of permissions to post in the channel, so log to stdout when some error happens,
            // with a description of it.
            println!("Received a ping request");
            // reply to the message onm discord

            if let Err(why) = msg.channel_id.say(&_ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
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
                commands::misc::ping::register(),
            ],
        )
            .await;
        println!("I created the following global slash command: {guild_command:#?}");
    }

    async fn typing_start(&self, _ctx: Context, event: TypingStartEvent) {
        println!("User {} started typing at {} in channel {}", event.user_id, event.timestamp, event.channel_id);
        // TODO: Implement the typing_start event
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