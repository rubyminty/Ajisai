mod commands;

use dotenv::dotenv;
use std::env;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Bot {
    database: sqlx::SqlitePool,
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "setup" => commands::setup::run(&command.data.options),
                _ => "not done lol :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let _pingcommand = Command::create_global_application_command(&ctx.http, |command| {
            commands::ping::register(command)
        })
        .await;

        let _setupcommand = Command::create_global_application_command(&ctx.http, |command| {
            commands::setup::register(command)
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a Token");

    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("db.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to DB");

    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run migrations!");

    let bot = Bot { database: database };

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(bot)
        .await
        .expect("Error creating client");

    if let Err(why_god_why) = client.start().await {
        println!("Client Error: {:#?}", why_god_why);
    }
}
