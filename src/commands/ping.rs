use serenity::client::Context;
use serenity::model::channel::Message;

pub async fn run(ctx: &Context, msg: &Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
    }
}