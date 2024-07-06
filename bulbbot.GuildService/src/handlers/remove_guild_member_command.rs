use crate::database::DbPool;
use crate::models::guilds::Guilds;
use crate::models::logging::Logging;
use crate::schema::guilds::dsl::*;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::{BelongingToDsl, QueryDsl, RunQueryDsl};
use lapin::Channel;

#[derive(Serialize, Deserialize)]
pub struct MessageDeleteLog {
    pub event: EventType,
    pub shard_id: u32,
    pub guild_id: Option<GuildId>,
    pub channel_id: ChannelId,
    pub deleted_message_id: MessageId,
    pub content: Option<String>,
}

pub async fn remove_guild_member_command_handler(
    request: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<i64>,
    channel: web::Data<Channel>,
    request_body: web::Json<MessageDeleteCommand>,
) -> actix_web::Result<impl Responder> {
    let guild_id = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().expect("Failed to get connection");

        let guild_query: Result<Guilds, _> = guilds.find(guild_id).get_result::<Guilds>(&mut conn);

        if guild_query.is_err() {
            return;
        }

        let guild = guild_query.unwrap();

        let logging_tables: Vec<Logging> = Logging::belonging_to(&guild)
            .load::<Logging>(&mut conn)
            .unwrap();
    })
    .await
    .expect("Blocking failed in guild create");

    Ok(HttpResponse::Ok())
}
