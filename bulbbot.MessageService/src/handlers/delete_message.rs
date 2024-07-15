use crate::models::messages::Messages;
use crate::schema::messages::dsl::messages;
use crate::{database::DbPool, http_client::HttpClient};
use actix_web::HttpRequest;
use actix_web::{http::Error, web, HttpResponse};
use common::telemetry::extractor_actix_web::ActixWebExtractor;
use common::telemetry::injector_rabbitmq::RabbitMqInjector;
use diesel::{QueryDsl, RunQueryDsl};
use lapin::options::BasicPublishOptions;
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel};
use models::event_type::EventType;
use models::message::mesage_delete_log::MessageDeleteLog;
use opentelemetry::global;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, GuildId, MessageId};
use tracing::debug;

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageDeleteEventContent {
    pub channel_id: ChannelId,
    pub deleted_message_id: MessageId,
    pub guild_id: Option<GuildId>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageDeleteCommand {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: u64,
    pub content: MessageDeleteEventContent,
}

pub async fn delete_message(
    request: HttpRequest,
    pool: web::Data<DbPool>,
    http_client: web::Data<HttpClient>,
    channel: web::Data<Channel>,
    request_body: web::Json<MessageDeleteCommand>,
) -> Result<HttpResponse, Error> {
    let response = http_client
        .get_guild(request_body.content.guild_id.unwrap(), request.headers())
        .await;

    if response.logging.message.is_none() {
        return Ok(HttpResponse::Ok().finish());
    }

    let message_id: i64 = request_body.content.deleted_message_id.into();

    let response: Option<Messages> = web::block(move || {
        let mut conn = pool.get().expect("Failed to get connection");

        let message_query: Result<Messages, _> =
            messages.find(message_id).get_result::<Messages>(&mut conn);

        if message_query.is_err() {
            return None;
        };

        Some(message_query.unwrap())
    })
    .await
    .expect("Blocking failed in add mesage");

    match response {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(msg) => {
            let cx = global::get_text_map_propagator(|propagator| {
                propagator.extract(&ActixWebExtractor {
                    headers: request.headers(),
                })
            });

            let mut headers = FieldTable::default();
            global::get_text_map_propagator(|propagator| {
                propagator.inject_context(&cx, &mut RabbitMqInjector(&mut headers))
            });

            let log = MessageDeleteLog {
                event: EventType::MessageDelete,
                guild_id: request_body.content.guild_id,
                channel_id: request_body.content.channel_id,
                deleted_message_id: request_body.content.deleted_message_id,
                shard_id: request_body.shard_id,
                content: msg.content,
            };

            let serialized =
                serde_json::to_string(&log).expect("[DELETE_MESSAGE] failed to serialize event");

            let payload = serialized.as_bytes();

            let confirm = channel
                .basic_publish(
                    "",
                    "bulbbot.logging",
                    BasicPublishOptions::default(),
                    payload,
                    BasicProperties::default().with_headers(headers),
                )
                .await
                .expect("[DELETE_MESSAGE] failed to publish to channel")
                .await
                .expect("[DELETE_MESSAGE] failed to get confirmation message from channel");

            debug!("Rabbit MQ channel publish return message: {:#?}", confirm);

            Ok(HttpResponse::Ok().finish())
        }
    }
}
