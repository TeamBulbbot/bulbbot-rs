use crate::database::DbPool;
use crate::models::guilds::Guilds;
use crate::models::logging::Logging;
use crate::schema::guilds::dsl::*;
use actix_web::{web, HttpResponse, Responder};
use diesel::{BelongingToDsl, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGuildResponse {
    #[serde(flatten)]
    pub guild: Guilds,
    pub logging: Logging,
}

pub async fn get_guild_command_handler(
    pool: web::Data<DbPool>,
    path: web::Path<i64>,
) -> actix_web::Result<impl Responder> {
    let guild_id = path.into_inner();

    let response = web::block(move || {
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
    .expect("Blocking failed in guild create");

    match response {
        Some(resp) => Ok(HttpResponse::Created().json(resp)),
        None => Ok(HttpResponse::NotFound().body(format!("Guild does not exist"))),
    }
}
