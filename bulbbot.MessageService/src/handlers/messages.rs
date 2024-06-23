use crate::{database::DbPool, http_client::HttpClient, models::event_type::EventType};
use actix_web::{http::Error, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serenity::model::channel::Message;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageCommand {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: i64,
    pub request_id: Uuid,
    pub content: Message,
}

pub async fn add_message(
    pool: web::Data<DbPool>,
    http_client: web::Data<HttpClient>,
    content: web::Json<MessageCommand>,
) -> Result<HttpResponse, Error> {
    let response = http_client
        .get_guild(content.content.guild_id.unwrap())
        .await;

    if response.logging.message.is_none() {
        return Ok(HttpResponse::Ok().finish());
    }

    Ok(HttpResponse::Ok().finish())
}
