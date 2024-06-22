use crate::models::event_type::EventType;
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

pub async fn add_message(content: web::Json<MessageCommand>) -> Result<HttpResponse, Error> {
    println!("{:#?}", content);

    Ok(HttpResponse::Ok().finish())
}
