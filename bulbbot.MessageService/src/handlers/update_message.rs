use crate::extractor::ActixWebExtractor;
use crate::injector::RabbitMqInjector;
use crate::models::messages::{Messages, NewMessage};
use crate::schema::messages::dsl::messages;
use crate::schema::messages::{content, message_id};
use crate::{database::DbPool, http_client::HttpClient, models::event_type::EventType};
use actix_web::HttpRequest;
use actix_web::{http::Error, web, HttpResponse};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use lapin::options::BasicPublishOptions;
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel};
use opentelemetry::global;
use serde::{Deserialize, Serialize};
use serenity::all::GuildId;
use serenity::model;
use tracing::debug;

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageUpdateCommand {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: u64,
    pub content: model::prelude::MessageUpdateEvent,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageUpdateLog {
    pub event: EventType,
    pub shard_id: u32,
    pub guild_id: Option<i64>,
    pub channel_id: i64,
    pub message_id: i64,
    pub before_content: Option<String>,
    pub after_content: Option<String>,
}

pub async fn update_message(
    request: HttpRequest,
    pool: web::Data<DbPool>,
    http_client: web::Data<HttpClient>,
    channel: web::Data<Channel>,
    request_body: web::Json<MessageUpdateCommand>,
) -> Result<HttpResponse, Error> {
    let response = http_client
        .get_guild(request_body.content.guild_id.unwrap(), request.headers())
        .await;

    if response.logging.message.is_none() {
        return Ok(HttpResponse::Ok().finish());
    }

    let req_message_id: i64 = request_body.content.id.into();
    let guild_id: Option<i64> = match request_body.content.guild_id {
        None => None,
        Some(g) => Some(g.into()),
    };
    let channel_id: i64 = request_body.content.channel_id.into();
    let shard_id = request_body.shard_id;
    let update_event = request_body.content.clone();

    let response: Option<Messages> = web::block(move || {
        let mut conn = pool.get().expect("Failed to get connection");

        let message_query: Result<Messages, _> = messages
            .find(req_message_id)
            .get_result::<Messages>(&mut conn);

        if message_query.is_err() {
            let new_message = NewMessage {
                message_id: req_message_id,
                guild_id: guild_id.unwrap_or_else(|| GuildId::new(1).into()),
                channel_id,
                author_id: request_body
                    .content
                    .author
                    .clone()
                    .unwrap_or_default()
                    .id
                    .into(),
                content: Some(
                    request_body
                        .content
                        .content
                        .clone()
                        .unwrap_or_else(|| String::new()),
                ),
            };

            diesel::insert_into(messages)
                .values(&new_message)
                .execute(&mut conn)
                .expect("Faild to insert guild into database");

            return None;
        };

        let _ = diesel::update(messages)
            .filter(message_id.eq(&req_message_id))
            .set(
                content.eq(request_body
                    .content
                    .content
                    .clone()
                    .unwrap_or_else(|| String::new())),
            )
            .execute(&mut conn);

        Some(message_query.unwrap())
    })
    .await
    .expect("Blocking failed in add mesage");

    match response {
        None => Ok(HttpResponse::Ok().finish()),
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

            let log = MessageUpdateLog {
                event: EventType::MessageUpdate,
                guild_id,
                channel_id,
                message_id: req_message_id,
                shard_id,
                before_content: msg.content,
                after_content: update_event.content,
            };

            let serialized =
                serde_json::to_string(&log).expect("[UPDATE_MESSAGE] failed to serialize event");

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
                .expect("[UPDATE_MESSAGE] failed to publish to channel")
                .await
                .expect("[UPDATE_MESSAGE] failed to get confirmation message from channel");

            debug!("Rabbit MQ channel publish return message: {:#?}", confirm);

            Ok(HttpResponse::Ok().finish())
        }
    }
}
