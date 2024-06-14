use crate::{
    events::{event_handler::Handler, models::event::Event},
    manger_container_structs::RabbitMQMangerContainer,
};
use lapin::{options::BasicPublishOptions, BasicProperties};
use serde::{Deserialize, Serialize};
use serenity::{
    model::prelude::{ChannelId, GuildId, MessageId},
    prelude::Context,
};
use tracing::debug;

#[derive(Serialize, Deserialize)]
pub struct MessageDeleteEventContent {
    pub channel_id: ChannelId,
    pub deleted_message_id: MessageId,
    pub guild_id: Option<GuildId>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageDeleteEvent {
    pub event: Event,
    pub shard_id: u32,
    pub timestamp: u64,
    pub content: MessageDeleteEventContent,
}

impl Handler {
    pub async fn handle_message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        deleted_message_id: MessageId,
        guild_id: Option<GuildId>,
    ) {
        let data = ctx.clone();
        let data_read = data.data.read().await;

        let channel = data_read
            .get::<RabbitMQMangerContainer>()
            .expect("[EVENT/MESSAGE_DELETE] failed to get the Rabbit MQ Channel");

        let event = MessageDeleteEvent {
            event: Event::MessageDelete,
            shard_id: ctx.shard_id.0,
            timestamp: Handler::get_unix_time(),
            content: MessageDeleteEventContent {
                channel_id,
                deleted_message_id,
                guild_id,
            },
        };
        let serlized = serde_json::to_string(&event)
            .expect("[EVENT/MESSAGE_DELETE] failed to serialize event");

        let payload = serlized.as_bytes();

        let confirm = channel
            .basic_publish(
                "",
                "bulbbot.gateway",
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await
            .expect("[EVENT/MESSAGE_DELETE] failed to publish to channel")
            .await
            .expect("[EVENT/MESSAGE_DELETE] failed to get confirmation message from channel");

        debug!("Rabbit MQ channel publish return message: {:#?}", confirm);
    }
}
