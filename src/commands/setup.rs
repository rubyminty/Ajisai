use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::{ChannelType};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("setup")
        .name_localized("de", "einrichten")
        .description("Set the bot up to receive confessions")
        .create_option(|option| {
            option
                .name("channel")
                .description("The channel to send confessions too")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
}

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected Channel Option")
        .resolved
        .as_ref()
        .expect("Expected Channel Object");

    if let CommandDataOptionValue::Channel(channel) = option {

        if channel.kind != ChannelType::Text {
            return "Please provide a valid Text Channel".to_string();
        }

        format!("Is the channel id: {}", channel.id.to_string())
    } else {
        "Oh fucky wucky".to_string()
    }
}