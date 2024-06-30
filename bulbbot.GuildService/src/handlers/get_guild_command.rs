use crate::models::guilds::Guilds;
use crate::models::logging::Logging;
use crate::schema::guilds::dsl::*;
use crate::{database::DbPool, extractor::ActixWebExtractor};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::{BelongingToDsl, QueryDsl, RunQueryDsl};
use opentelemetry::global::{self, ObjectSafeSpan};
use opentelemetry::trace::{Tracer, TracerProvider};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGuildResponse {
    #[serde(flatten)]
    pub guild: Guilds,
    pub logging: Logging,
}

pub async fn get_guild_command_handler(
    request: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<i64>,
) -> actix_web::Result<impl Responder> {
    let cx = global::get_text_map_propagator(|propagator| {
        propagator.extract(&mut ActixWebExtractor {
            headers: &mut request.headers(),
        })
    });

    let tracer_provider = global::tracer_provider();

    let tracer = tracer_provider
        .tracer_builder("get_guild")
        .with_version(env!("CARGO_PKG_VERSION"))
        .build();

    let mut span = tracer.start_with_context("get_guild", &cx);

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

    span.end();
    match response {
        Some(resp) => Ok(HttpResponse::Created().json(resp)),
        None => Ok(HttpResponse::NotFound().body(format!("Guild does not exist"))),
    }
}
