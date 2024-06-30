use crate::{
    events::{event_handler::Handler, models::event::Event},
    manger_container_structs::RabbitMQMangerContainer,
    rabbit_mq::RabbitMqInjector,
};
use lapin::{options::BasicPublishOptions, types::FieldTable, BasicProperties};
use opentelemetry::{
    global::{self, ObjectSafeSpan},
    trace::{SpanKind, Status, TraceContextExt, Tracer},
    KeyValue,
};
use serde::{Deserialize, Serialize};
use serenity::prelude::Context;
use serenity::{all::GuildId, model::prelude::MessageUpdateEvent};
use tracing::debug;

#[derive(Serialize, Deserialize)]
pub struct MessageDeleteEvent {
    pub event: Event,
    pub shard_id: u32,
    pub timestamp: u64,
    pub content: MessageUpdateEvent,
}

impl Handler {
    pub async fn handle_message_update(&self, ctx: Context, event: MessageUpdateEvent) {
        let tracer = global::tracer(String::new());

        let mut span = tracer
            .span_builder("message_update")
            .with_kind(SpanKind::Producer)
            .start(&tracer);

        span.set_attribute(KeyValue::new(
            "guild_id",
            event
                .guild_id
                .unwrap_or_else(|| GuildId::new(1))
                .to_string(),
        ));
        span.set_attribute(KeyValue::new("channel_id", event.channel_id.to_string()));
        span.set_attribute(KeyValue::new("message_id", event.id.to_string()));
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
            .expect("[EVENT/MESSAGE_UPDATE] failed to get the Rabbit MQ Channel");

        let event = MessageDeleteEvent {
            event: Event::MessageUpdate,
            shard_id: ctx.shard_id.0,
            timestamp: Handler::get_unix_time(),
            content: event,
        };
        let serlized = serde_json::to_string(&event)
            .expect("[EVENT/MESSAGE_UPDATE] failed to serialize event");

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
            .expect("[EVENT/MESSAGE_UPDATE] failed to publish to channel")
            .await
            .expect("[EVENT/MESSAGE_UPDATE] failed to get confirmation message from channel");

        debug!("Rabbit MQ channel publish return message: {:#?}", confirm);

        cx.span().set_status(Status::Ok);
        cx.span().end();
    }
}
