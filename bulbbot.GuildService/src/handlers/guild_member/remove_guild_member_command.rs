use crate::database::DbPool;
use crate::models::logging::Logging;
use crate::schema::logging::dsl::logging;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use common::telemetry::extractor_actix_web::ActixWebExtractor;
use common::telemetry::injector_rabbitmq::RabbitMqInjector;
use diesel::query_dsl::methods::FindDsl;
use diesel::RunQueryDsl;
use lapin::options::BasicPublishOptions;
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel};
use models::event_type::EventType;
use models::guild::guild_member::guild_member_add_log::AddGuildMemberLog;
use models::guild::guild_member::guild_member_addition_event::GuildMemberAdditionEventContent;
use models::guild::guild_member::guild_member_remove_log::RemoveGuildMemberLog;
use opentelemetry::global;
use serde::{Deserialize, Serialize};
use serenity::all::{GuildId, User};
use tracing::debug;

#[derive(Serialize, Deserialize)]
pub struct RemoveGuildMemberContent {
    pub guild_id: GuildId,
    pub user: User,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveGuildMemberCommand {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: u64,
    pub content: RemoveGuildMemberContent,
}

pub async fn remove_guild_member_command_handler(
    request: HttpRequest,
    pool: web::Data<DbPool>,
    channel: web::Data<Channel>,
    request_body: web::Json<RemoveGuildMemberCommand>,
) -> actix_web::Result<impl Responder> {
    let guild_id: i64 = request_body.content.guild_id.into();
    let response = web::block(move || {
        let mut conn = pool.get().expect("Failed to get connection");

        let log_table: Result<Logging, _> = logging.find(guild_id).get_result::<Logging>(&mut conn);

        if log_table.is_err() {
            return None;
        }

        Some(log_table.unwrap())
    })
    .await
    .expect("Blocking failed in guild create");

    if response.is_none() {
        return Ok(HttpResponse::NotFound());
    }
    let response = response.unwrap();

    if response.join_leave.is_none() {
        return Ok(HttpResponse::Ok());
    }

    let cx = global::get_text_map_propagator(|propagator| {
        propagator.extract(&ActixWebExtractor {
            headers: request.headers(),
        })
    });

    let mut headers = FieldTable::default();
    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&cx, &mut RabbitMqInjector(&mut headers))
    });

    let log = RemoveGuildMemberLog {
        event: EventType::GuildMemberRemoval,
        shard_id: request_body.shard_id,
        guild_id: request_body.content.guild_id,
        user_id: request_body.content.user.id,
        content: request_body.content.user.clone(),
    };

    let serialized =
        serde_json::to_string(&log).expect("[REMOVE_GUILD_MEMBER] failed to serialize event");

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
        .expect("[REMOVE_GUILD_MEMBER] failed to publish to channel")
        .await
        .expect("[REMOVE_GUILD_MEMBER] failed to get confirmation message from channel");

    debug!("Rabbit MQ channel publish return message: {:#?}", confirm);

    Ok(HttpResponse::Ok())
}
