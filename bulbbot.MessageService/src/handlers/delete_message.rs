use crate::schema::messages::dsl::messages;
use crate::{
    database::DbPool,
    http_client::HttpClient,
    models::{event_type::EventType, messages::NewMessage},
};
use actix_web::{http::Error, web, HttpResponse};
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, GuildId, MessageId};

#[derive(Serialize, Deserialize)]
pub struct MessageDeleteEventContent {
    pub channel_id: ChannelId,
    pub deleted_message_id: MessageId,
    pub guild_id: Option<GuildId>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageDeleteCommand {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: u64,
    pub content: MessageDeleteEventContent,
}

pub async fn delete_message(
    pool: web::Data<DbPool>,
    http_client: web::Data<HttpClient>,
    content: web::Json<MessageDeleteCommand>,
) -> Result<HttpResponse, Error> {
    /* let response = http_client
        .get_guild(content.content.guild_id.unwrap())
        .await;

    if response.logging.message.is_none() {
        return Ok(HttpResponse::Ok().finish());
    } */

    /*web::block(move || {
           let mut conn = pool.get().expect("Failed to get connection");
    let guild_query: Result<Guilds, _> = guilds.find(guild_id).get_result::<Guilds>(&mut conn);

           if guild_query.is_err() {
               return None;
           }

           let guild = guild_query.unwrap();

           let logging_tables: Vec<Logging> = Logging::belonging_to(&guild)
               .load::<Logging>(&mut conn)
               .unwrap();

           Some(CreateGuildResponse {
               guild,
               logging: *logging_tables.first().expect("Logging tables is empty"),
           })
       })
       .await
       .expect("Blocking failed in guild create");*/

    Ok(HttpResponse::Ok().finish())
}
