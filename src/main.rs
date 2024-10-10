use serenity::async_trait;
use serenity::model::gateway::Activity;
use serenity::gateway::ActivityData;
use serenity::model::gateway::Ready;
use serenity::all::GatewayIntents;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::user::OnlineStatus;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("~") {
            handle_command(&ctx, &msg).await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        ctx.set_activity(Some(ActivityData::playing("with serenity-rs")));
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

async fn handle_command(ctx: &Context, msg: &Message) {
    let command_name = msg.content.split_whitespace().next().unwrap_or("").trim_start_matches('~');
    match command_name {
        "ping" => commands::ping::run(ctx, msg).await,
        _ => {
            let _ = msg.channel_id.say(&ctx.http, "Unknown command.").await;
        }
    }
}

mod commands {
    pub mod ping;
}