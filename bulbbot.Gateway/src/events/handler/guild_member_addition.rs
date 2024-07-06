use crate::events::event_handler::Handler;
use crate::events::models::event::Event;
use crate::manger_container_structs::RabbitMQMangerContainer;
use crate::rabbit_mq::RabbitMqInjector;
use lapin::types::FieldTable;
use lapin::{options::BasicPublishOptions, BasicProperties};
use opentelemetry::global::ObjectSafeSpan;
use opentelemetry::trace::{SpanKind, Status, TraceContextExt};
use opentelemetry::KeyValue;
use opentelemetry::{global, trace::Tracer};
use serde::{Deserialize, Serialize};
use serenity::all::Member;
use serenity::prelude::Context;
use tracing::debug;

#[derive(Serialize, Deserialize)]
pub struct GuildMemberAdditionEvent {
    pub event: Event,
    pub shard_id: u32,
    pub timestamp: u64,
    pub content: Member,
}

impl Handler {
    pub async fn handle_guild_member_addition(&self, ctx: Context, member: Member) {
        let tracer = global::tracer(String::new());

        let mut span = tracer
            .span_builder("guild_member_addition")
            .with_kind(SpanKind::Producer)
            .start(&tracer);

        span.set_attribute(KeyValue::new("guild_id", member.guild_id.to_string()));
        span.set_attribute(KeyValue::new("user_id", member.user.id.to_string()));
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
            .expect("[EVENT/GUILD_MEMBER_ADDITION] failed to get the Rabbit MQ Channel");

        let event = GuildMemberAdditionEvent {
            event: Event::GuildMemberAddition,
            shard_id: ctx.shard_id.0,
            timestamp: Handler::get_unix_time(),
            content: member,
        };
        let serialized = serde_json::to_string(&event)
            .expect("[EVENT/GUILD_MEMBER_ADDITION] failed to serialize event");

        let payload = serialized.as_bytes();

        let confirm = channel
            .basic_publish(
                "",
                "bulbbot.gateway",
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default().with_headers(headers),
            )
            .await
            .expect("[EVENT/GUILD_MEMBER_ADDITION] failed to publish to channel")
            .await
            .expect(
                "[EVENT/GUILD_MEMBER_ADDITION] failed to get confirmation message from channel",
            );

        debug!("Rabbit MQ channel publish return message: {:#?}", confirm);

        cx.span().set_status(Status::Ok);
        cx.span().end();
    }
}
