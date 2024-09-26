use serenity::all::{CommandOptionType, CreateCommand, ResolvedOption};
use serenity::all::RoleAction::Create;
use serenity::builder::{CreateCommandOption, CreateForumPost};
use serenity::model::Permissions;

pub fn run(options: &[ResolvedOption]) -> String {
    "Ping!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("join_auto_role")
        .description("Replies with Pong!!")
        .dm_permission(false)
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Role,
                "select_role",
                "Replies with Pong!!",
            ).required(true),
        )
}