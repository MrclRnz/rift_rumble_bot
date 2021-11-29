//use std::env;

use rift_rumble_bot::entities::{entity::RiftRumbleEntityCollection, entity_loader};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    let mut entity_collection = RiftRumbleEntityCollection::new();
    if let Err(e) = entity_loader::load_static_data(&mut entity_collection) {
        panic!(
            "Error while loading all static data from data dragon: {}",
            e
        );
    }

    let champion = entity_collection.randomize_champion();
    let items = entity_collection.randomize_items();
    let runes = entity_collection.randomize_runes();
    let summoners = entity_collection.randomize_summoners();

    println!("Champion: {:?}", champion);
    println!("Items: {:?}", items);
    println!("Runes: {:?}", runes);
    println!("Summoners: {:?}", summoners);

    //init_bot();
}

#[tokio::main]
async fn init_bot() {
    // Configure the client with your Discord bot token in the environment.
    //let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let token = "123";

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
