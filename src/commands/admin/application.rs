use serenity::all::{CommandOptionType, CreateCommandOption, Permissions, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

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
                    value: ResolvedValue::Boolean(player),
                    ..
                }) = sub_command.first()
                {
                    format!(
                        "Application system is now {}",
                        if *player { "enabled" } else { "disabled" }
                    )
                } else {
                    "Invalid option".to_string()
                }
            }
            "recruitment" => {
                if let Some(ResolvedOption {
                    value: ResolvedValue::Boolean(player),
                    ..
                }) = sub_command.first()
                {
                    format!(
                        "Recruitment is now {}",
                        if *player { "enabled" } else { "disabled" }
                    )
                } else {
                    "Invalid option".to_string()
                }
            }
            "channel" => {
                if let Some(ResolvedOption {
                    value: ResolvedValue::Channel(channel),
                    ..
                }) = sub_command.first()
                {
                    format!("Channel is now set to <#{}> {}", channel.id, channel.id)
                } else {
                    "Invalid option".to_string()
                }
            }
            "role-member" => {
                if let Some(ResolvedOption {
                    value: ResolvedValue::Role(role),
                    ..
                }) = sub_command.first()
                {
                    format!("Role-member is now set to <@&{}> {}", role.id, role.id)
                } else {
                    "Invalid option".to_string()
                }
            }
            "role-manager" => {
                if let Some(ResolvedOption {
                    value: ResolvedValue::Role(role),
                    ..
                }) = sub_command.first()
                {
                    format!("Role-manager is now set to <@&{}> {}", role.id, role.id)
                } else {
                    "Invalid option".to_string()
                }
            }
            _ => "Invalid subcommand".to_string(),
        }
    } else {
        "Invalid subcommand".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("application")
        .description("Manage application system")
        .dm_permission(false)
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "enabled",
                "Enable or disable the application system",
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
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "recruitment",
                "Let an User create an application",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::Boolean,
                    "open",
                    "true to enable, false to disable",
                )
                .required(true),
            ),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "channel",
                "Set the channel where the application will be sent",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::Channel,
                    "channel",
                    "Select the channel",
                )
                .required(true),
            ),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "role-member",
                "Set the role that will be given to the user after the application",
            )
            .add_sub_option(
                CreateCommandOption::new(CommandOptionType::Role, "role", "Select the role")
                    .required(true),
            ),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "role-manager",
                "Set the role that will be given to the staff after the application",
            )
            .add_sub_option(
                CreateCommandOption::new(CommandOptionType::Role, "role", "Select the role")
                    .required(true),
            ),
        )
}