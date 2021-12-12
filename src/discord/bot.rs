use std::{collections::HashMap, env, sync::Mutex};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::ChannelId, user::User},
    prelude::*,
};

use crate::entities::entity::RiftRumbleEntityCollection;
use crate::entities::entity::RiftRumbleEntitySet;

struct Handler {
    rift_rumble_participants: Mutex<HashMap<User, RiftRumbleEntitySet>>,
    entity_collection: RiftRumbleEntityCollection
}

impl Handler {
    async fn send_public_message(channel: &ChannelId, ctx: &Context, message: &str) {
        if let Err(why) = channel.say(&ctx.http, message).await {
            println!("Error sending message: {:?}", why);
        }
    }

    async fn send_private_message(user: &User, ctx: &Context, message: &str) {
        if let Ok(private_channel) = user.create_dm_channel(&ctx.http).await {
            if let Err(why) = private_channel.say(&ctx.http, message).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&'static self, ctx: Context, msg: Message) {
        if msg.content == "!open" {
            if let Ok(map) = self.rift_rumble_participants.lock().as_mut() {
                map.clear();
            }
            Handler::send_public_message(&msg.channel_id, &ctx, "Rift Rumble initialized!");
        }
        if msg.content == "!participate" {
            if let Ok(map) = self.rift_rumble_participants.lock().as_mut() {
                map.entry(msg.author).or_insert(self.entity_collection.randomize_set());
            }
            Handler::send_private_message(
                &msg.author,
                &ctx,
                "You have been added to the Rift Rumble!",
            );
        }
        if msg.content == "!leave" {
            if let Ok(map) = self.rift_rumble_participants.lock().as_mut() {
                map.remove(&msg.author);
            }
            Handler::send_private_message(
                &msg.author,
                &ctx,
                "You have been removed from the Rift Rumble!",
            );
        }
        if msg.content == "!start" {

        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

//#[tokio::main]
pub async fn init_bot(entity_collection: RiftRumbleEntityCollection) {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler {
            rift_rumble_participants: Mutex::new(HashMap::new()),
            entity_collection: entity_collection
        })
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
