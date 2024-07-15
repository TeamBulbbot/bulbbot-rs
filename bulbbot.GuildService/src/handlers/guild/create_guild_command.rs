use crate::database::DbPool;
use crate::models::guilds::{Guilds, NewGuild};
use crate::models::logging::{Logging, NewLogging};
use crate::schema::guilds::dsl::*;
use crate::schema::logging::dsl::*;
use actix_web::{web, HttpResponse, Responder};
use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGuildCommand {
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGuildResponse {
    #[serde(flatten)]
    pub guild: Guilds,
    pub logging: Logging,
}

pub async fn create_guild_command_handler(
    pool: web::Data<DbPool>,
    content: web::Json<CreateGuildCommand>,
) -> actix_web::Result<impl Responder> {
    let response = web::block(move || {
        let mut conn = pool.get().expect("Failed to get connection");

        let existing = guilds.filter(id.eq(content.id)).first::<Guilds>(&mut conn);
        if existing.is_ok() {
            return None;
        }

        let new_guild = NewGuild { id: content.id };

        let new_guild_logging = NewLogging {
            guilds_id: content.id,
        };

        diesel::insert_into(guilds)
            .values(&new_guild)
            .execute(&mut conn)
            .expect("Faild to insert guild into database");

        diesel::insert_into(logging)
            .values(&new_guild_logging)
            .execute(&mut conn)
            .expect("Faild to insert logging into database");

        let guild: Guilds = guilds
            .find(content.id)
            .get_result::<Guilds>(&mut conn)
            .unwrap();

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
        None => Ok(HttpResponse::BadRequest().body("Guild already exists")),
    }
}
