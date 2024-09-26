use serenity::all::{CommandOptionType, CreateCommand, ResolvedOption, ResolvedValue};
use serenity::builder::CreateCommandOption;
use serenity::model::Permissions;

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
                    value: ResolvedValue::SubCommand(sub_command),
                    name: sub_command_name,
                    ..
                }) = options.first()
    {
        match sub_command_name.to_string().as_str() {
            "enabled" => {
                if let Some(ResolvedOption {
                                value: ResolvedValue::Boolean(status),
                                ..
                            }) = sub_command.first()
                {
                    format!(
                        "Application system is now {}",
                        if *status { "enabled" } else { "disabled" }
                    )
                } else {
                    "Invalid option".to_string()
                }
            }
            _ => {
                "Invalid option".to_string()
            }
        }
    } else {
        "Invalid option".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("join_auto_role")
        .description("Manage join auto role system")
        .dm_permission(false)
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "enabled",
                "Enable or disable the join auto role system",
            )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Boolean,
                        "enabled",
                        "true to enable, false to disable",
                    )
                        .required(true),
                ),
        )
}