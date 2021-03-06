use crate::entities::entity::CustomHashMap;
use crate::entities::entity::RiftRumbleEntity;
use crate::entities::entity::RiftRumbleEntityCollection;
use serenity::builder::CreateMessage;
use serenity::model::prelude::UserId;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::ChannelId},
    prelude::*,
};
use std::{collections::HashMap, env, sync::Arc};

struct Handler;

impl Handler {
    async fn send_public_message(channel: &ChannelId, ctx: &Context, message: &str) {
        if let Err(why) = channel.say(&ctx.http, message).await {
            println!("Error sending message: {:?}", why);
        }
    }

    async fn send_private_message(user: &UserId, ctx: &Context, message: &str) {
        if let Ok(private_channel) = user.create_dm_channel(&ctx.http).await {
            if let Err(why) = private_channel.say(&ctx.http, message).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn send_set_as_private_message(ctx: &Context) {
        let user_map = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<CustomHashMap>()
                .expect("Expected CustomHashMap in TypeMap.")
                .clone()
        };
        {
            let map = user_map.write().await;
            for (key, value) in map.0.iter() {
                println!("User: {}", key);
                let mut embed = CreateMessage::default();

                if let RiftRumbleEntity::Champion(c) = value.champion.as_ref() {
                    embed.add_embed(|e| e.title("Champion").description(&c.name));
                }

                let mut message_string = "".to_owned();
                for item in &value.items {
                    if let RiftRumbleEntity::Item(i, _m) = item.as_ref() {
                        message_string.push_str(&i.name);
                        message_string.push_str("\r\n");
                    }
                }
                embed.add_embed(|e| e.title("Items").description(message_string));

                message_string = "".to_owned();
                if let RiftRumbleEntity::Summoner(s) = &value.summoners.0.as_ref() {
                    message_string.push_str(&s.name);
                    message_string.push_str("\r\n");
                }
                if let RiftRumbleEntity::Summoner(s) = &value.summoners.1.as_ref() {
                    message_string.push_str(&s.name);
                }
                embed.add_embed(|e| e.title("Summoners").description(message_string));

                message_string = "".to_owned();
                message_string.push_str(&value.skill_order.0);
                message_string.push_str(" -> ");
                message_string.push_str(&value.skill_order.1);
                message_string.push_str(" -> ");
                message_string.push_str(&value.skill_order.2);
                embed.add_embed(|e| e.title("Skill Order").description(message_string));

                for (tree, rune_set) in value.runes.iter() {
                    message_string = "".to_owned();
                    for rune in rune_set {
                        if let RiftRumbleEntity::Rune(r) = rune.as_ref() {
                            message_string.push_str(&r.name);
                            message_string.push_str("\r\n");
                        }
                    }
                    embed.add_embed(|e| e.title("Runes").field(tree, message_string, true));
                }

                /*
                 embed.add_embed(|e| {
                                    e.title("Champion")
                                        .fields(vec![
                                            ("This is the first field", "This is a field body", true),
                                            ("This is the second field", "Both fields are inline", true),
                                        ])
                                        .field(
                                            "This is the third field",
                                            "This is not an inline field",
                                            false,
                                        )
                                        .footer(|f| f.text("This is a footer"))
                                });
                */

                if let Ok(private_channel) = key.create_dm_channel(&ctx.http).await {
                    if let Err(why) = private_channel
                        .send_message(&ctx.http, |_| &mut embed)
                        .await
                    {
                        println!("Error sending message: {:?}", why);
                    };
                }
            }
        }
    }

    async fn clear_participants(ctx: &Context) {
        let user_map = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<CustomHashMap>()
                .expect("Expected CustomHashMap in TypeMap.")
                .clone()
        };
        {
            let mut map = user_map.write().await;
            map.0.clear();
        }
    }

    async fn generate_set_for_user(ctx: &Context, msg: &Message) {
        let collection = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<RiftRumbleEntityCollection>()
                .expect("Expected RiftRumbleEntityCollection in TypeMap.")
                .clone()
        };

        let user_map = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<CustomHashMap>()
                .expect("Expected CustomHashMap in TypeMap.")
                .clone()
        };

        {
            let mut map_lock = user_map.write().await;
            map_lock.0.insert(msg.author.id, collection.randomize_set());
            /*
            println!("{:?}", map_lock.0);
            Handler::send_private_message(
                &msg.author,
                &ctx,
                format!("{:?}", map_lock.0).as_str(),
            ).await;
            */
        }
    }

    async fn remove_participant(ctx: &Context, msg: &Message) {
        let user_map = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<CustomHashMap>()
                .expect("Expected CustomHashMap in TypeMap.")
                .clone()
        };
        {
            let mut map = user_map.write().await;
            map.0.remove(&msg.author.id);
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!open" {
            Handler::clear_participants(&ctx).await;
            Handler::send_public_message(&msg.channel_id, &ctx, "Rift Rumble initialized!").await;
        }
        if msg.content == "!participate" {
            Handler::generate_set_for_user(&ctx, &msg).await;
            Handler::send_private_message(
                &msg.author.id,
                &ctx,
                "You have been added to the Rift Rumble!",
            )
            .await;
        }
        if msg.content == "!leave" {
            Handler::remove_participant(&ctx, &msg).await;
            Handler::send_private_message(
                &msg.author.id,
                &ctx,
                "You have been removed from the Rift Rumble!",
            )
            .await;
        }
        if msg.content == "!start" {
            Handler::send_set_as_private_message(&ctx).await;
        }
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
