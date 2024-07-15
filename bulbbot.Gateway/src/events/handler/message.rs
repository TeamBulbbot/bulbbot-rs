use crate::events::event_handler::Handler;
use crate::manger_container_structs::RabbitMQMangerContainer;
use common::telemetry::injector_rabbitmq::RabbitMqInjector;
use lapin::types::FieldTable;
use lapin::{options::BasicPublishOptions, BasicProperties};
use models::event_type::EventType;
use models::message::message_event::MessageEvent;
use opentelemetry::global::ObjectSafeSpan;
use opentelemetry::trace::{SpanKind, Status, TraceContextExt};
use opentelemetry::KeyValue;
use opentelemetry::{global, trace::Tracer};
use serenity::all::GuildId;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use tracing::debug;

impl Handler {
    pub async fn handle_message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.author.system || msg.guild_id.is_none() {
            return;
        }

        let tracer = global::tracer(String::new());

        let mut span = tracer
            .span_builder("message")
            .with_kind(SpanKind::Producer)
            .start(&tracer);

        span.set_attribute(KeyValue::new(
            "guild_id",
            msg.guild_id.unwrap_or_else(|| GuildId::new(1)).to_string(),
        ));
        span.set_attribute(KeyValue::new("channel_id", msg.channel_id.to_string()));
        span.set_attribute(KeyValue::new("message_id", msg.id.to_string()));
        span.set_attribute(KeyValue::new("author_id", msg.author.id.to_string()));
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
            .expect("[EVENT/MESSAGE] failed to get the Rabbit MQ Channel");

        let event = MessageEvent {
            event: EventType::Message,
            shard_id: ctx.shard_id.0,
            timestamp: Handler::get_unix_time(),
            content: msg,
        };
        let serialized =
            serde_json::to_string(&event).expect("[EVENT/MESSAGE] failed to serialize event");

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
            .expect("[EVENT/MESSAGE] failed to publish to channel")
            .await
            .expect("[EVENT/MESSAGE] failed to get confirmation message from channel");

        debug!("Rabbit MQ channel publish return message: {:#?}", confirm);

        cx.span().set_status(Status::Ok);
        cx.span().end();
    }
}
