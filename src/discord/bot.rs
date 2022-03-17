use crate::entities::entity::CustomHashMap;
use std::{collections::HashMap, env, sync::Arc};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::ChannelId, user::User},
    prelude::*,
};

use crate::entities::entity::RiftRumbleEntityCollection;

struct Handler;

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
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!open" {
            let user_map = {
                let data_read = ctx.data.read().await;
                data_read.get::<CustomHashMap>().expect("Expected CustomHashMap in TypeMap.").clone()
            };
            {
                let mut map = user_map.write().await;
                map.0.clear();
            }
            Handler::send_public_message(&msg.channel_id, &ctx, "Rift Rumble initialized!").await;
        }
        if msg.content == "!participate" {

            let collection = {
                let data_read = ctx.data.read().await;
                data_read.get::<RiftRumbleEntityCollection>().expect("Expected RiftRumbleEntityCollection in TypeMap.").clone()
            };

            let user_map = {
                let data_read = ctx.data.read().await;
                data_read.get::<CustomHashMap>().expect("Expected CustomHashMap in TypeMap.").clone()
            };
        
            {
                let mut map_lock = user_map.write().await;
                map_lock.0.insert(msg.author.id, collection.randomize_set());
                println!("{:?}", map_lock.0);
                Handler::send_private_message(
                    &msg.author,
                    &ctx,
                    format!("{:?}", map_lock.0).as_str(),
                ).await;
            }
        
            Handler::send_private_message(
                &msg.author,
                &ctx,
                "You have been added to the Rift Rumble!",
            ).await;
        }
        if msg.content == "!leave" {
            Handler::send_private_message(
                &msg.author,
                &ctx,
                "You have been removed from the Rift Rumble!",
            ).await;
        }
        if msg.content == "!start" {}
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
pub async fn init_bot(entity_collection: RiftRumbleEntityCollection) {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<RiftRumbleEntityCollection>(Arc::new(entity_collection));
        data.insert::<CustomHashMap>(Arc::new(RwLock::new(CustomHashMap(HashMap::new()))));
    }
    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
