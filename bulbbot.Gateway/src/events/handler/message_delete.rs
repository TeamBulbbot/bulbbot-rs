use crate::{
    events::{event_handler::Handler, models::event::Event},
    manger_container_structs::RabbitMQMangerContainer,
    rabbit_mq::RabbitMqInjector,
};
use lapin::{options::BasicPublishOptions, types::FieldTable, BasicProperties};
use opentelemetry::{
    global,
    trace::{SpanKind, Status, TraceContextExt, Tracer},
};
use opentelemetry::{global::ObjectSafeSpan, KeyValue};
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
        let tracer = global::tracer(String::new());

        let mut span = tracer
            .span_builder("message_delete")
            .with_kind(SpanKind::Producer)
            .start(&tracer);

        span.set_attribute(KeyValue::new(
            "guild_id",
            guild_id.unwrap_or_else(|| GuildId::new(1)).to_string(),
        ));
        span.set_attribute(KeyValue::new("channel_id", channel_id.to_string()));
        span.set_attribute(KeyValue::new("message_id", deleted_message_id.to_string()));
        span.set_attribute(KeyValue::new("shard_id", ctx.shard_id.0.to_string()));

        let cx = opentelemetry::Context::current_with_span(span);

        let mut headers = FieldTable::default();
        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&cx, &mut RabbitMqInjector(&mut headers))
        });

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
                BasicProperties::default().with_headers(headers),
            )
            .await
            .expect("[EVENT/MESSAGE_DELETE] failed to publish to channel")
            .await
            .expect("[EVENT/MESSAGE_DELETE] failed to get confirmation message from channel");

        debug!("Rabbit MQ channel publish return message: {:#?}", confirm);

        cx.span().set_status(Status::Ok);
        cx.span().end();
    }
}
