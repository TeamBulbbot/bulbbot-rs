use crate::extractor::ActixWebExtractor;
use crate::schema::messages::dsl::messages;
use crate::{
    database::DbPool,
    http_client::HttpClient,
    models::{event_type::EventType, messages::NewMessage},
};
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::{http::Error, HttpResponse};
use diesel::RunQueryDsl;
use opentelemetry::global;
use opentelemetry::trace::{Span, Tracer, TracerProvider};
use serde::{Deserialize, Serialize};
use serenity::{all::GuildId, model::channel::Message};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageCommand {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: i64,
    pub content: Message,
}

pub async fn create_message(
    request: HttpRequest,
    pool: web::Data<DbPool>,
    http_client: web::Data<HttpClient>,
    content: web::Json<MessageCommand>,
) -> Result<HttpResponse, Error> {
    let cx = global::get_text_map_propagator(|propagator| {
        propagator.extract(&mut ActixWebExtractor {
            headers: &mut request.headers(),
        })
    });

    let tracer_provider = global::tracer_provider();

    let tracer = tracer_provider
        .tracer_builder("create_message")
        .with_version(env!("CARGO_PKG_VERSION"))
        .build();

    let mut span = tracer.start_with_context("create_message", &cx);

    let response = http_client
        .get_guild(content.content.guild_id.unwrap(), &cx)
        .await;

    if response.logging.message.is_none() {
        span.end();
        return Ok(HttpResponse::Ok().finish());
    }

    web::block(move || {
        let mut conn = pool.get().expect("Failed to get connection");

        let new_message = NewMessage {
            message_id: content.content.id.into(),
            guild_id: content
                .content
                .guild_id
                .unwrap_or_else(|| GuildId::new(1))
                .into(),
            channel_id: content.content.channel_id.into(),
            author_id: content.content.author.id.into(),
            content: Some(content.content.content.clone()),
        };

        diesel::insert_into(messages)
            .values(&new_message)
            .execute(&mut conn)
            .expect("Faild to insert guild into database");
    })
    .await
    .expect("Blocking failed in add mesage");

    span.end();

    Ok(HttpResponse::Ok().finish())
}
