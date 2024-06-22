use crate::events::event_handler::Handler;
use crate::events::models::event::Event;
use crate::manger_container_structs::RabbitMQMangerContainer;
use lapin::{options::BasicPublishOptions, BasicProperties};
use serde::{Deserialize, Serialize};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use tracing::debug;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct MessageEvent {
    pub event: Event,
    pub shard_id: u32,
    pub timestamp: u64,
    pub request_id: Uuid,
    pub content: Message,
}

impl Handler {
    pub async fn handle_message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.author.system {
            return;
        }

        let data = ctx.clone();
        let data_read = data.data.read().await;

        let channel = data_read
            .get::<RabbitMQMangerContainer>()
            .expect("[EVENT/MESSAGE] failed to get the Rabbit MQ Channel");

        let event = MessageEvent {
            event: Event::Message,
            shard_id: ctx.shard_id.0,
            timestamp: Handler::get_unix_time(),
            request_id: Uuid::new_v4(),
            content: msg,
        };
        let serlized =
            serde_json::to_string(&event).expect("[EVENT/MESSAGE] failed to serialize event");

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
            .expect("[EVENT/MESSAGE] failed to publish to channel")
            .await
            .expect("[EVENT/MESSAGE] failed to get confirmation message from channel");

        debug!("Rabbit MQ channel publish return message: {:#?}", confirm);
    }
}
