use actix_web::web;

use crate::handlers::create_guild_command;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/guilds").service(
        web::resource("").route(web::post().to(create_guild_command::create_guild_command_handler)),
    ));
}
