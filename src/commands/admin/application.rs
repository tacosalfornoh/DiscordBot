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
        match sub_command_name {
            &"enabled" => {
                if let Some(ResolvedOption {
                    value: ResolvedValue::Boolean(player),
                    ..
                }) = sub_command.first()
                {
                    // TODO: Implement the application system
                    return format!(
                        "Application system is now {}",
                        if *player { "enabled" } else { "disabled" }
                    );
                } else {
                    return "Invalid option".to_string();
                }
            }
            &"recruitment" => {
                if let Some(ResolvedOption {
                    value: ResolvedValue::Boolean(player),
                    ..
                }) = sub_command.first()
                {
                    // TODO: Implement the recruitment system
                    return format!(
                        "Recruitment is now {}",
                        if *player { "enabled" } else { "disabled" }
                    );
                } else {
                    return "Invalid option".to_string();
                }
            }
            &"channel" => {
                if let Some(ResolvedOption {
                    value: ResolvedValue::Channel(channel),
                    ..
                }) = sub_command.first()
                {
                    // TODO: Implement the channel system
                    return format!("Channel is now set to <#&{}> {}", channel.id, channel.id);
                } else {
                    return "Invalid option".to_string();
                }
            }
            &"role-member" => {
                if let Some(ResolvedOption {
                    value: ResolvedValue::Role(role),
                    ..
                }) = sub_command.first()
                {
                    // TODO: Implement the role-member system
                    return format!("Role-member is now set to <@&{}> {}", role.id, role.id);
                } else {
                    return "Invalid option".to_string();
                }
            }
            &"role-manager" => {
                if let Some(ResolvedOption {
                    value: ResolvedValue::Role(role),
                    ..
                }) = sub_command.first()
                {
                    // TODO: Implement the role-manager system
                    return format!("Role-manager is now set to <@&{}> {}", role.id, role.id);
                } else {
                    return "Invalid option".to_string();
                }
            }
            _ => return "Invalid subcommand".to_string(),
        }
    } else {
        return "Invalid subcommand".to_string();
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
