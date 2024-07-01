use actix_web::web;

use crate::handlers::*;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/guilds")
            .service(
                web::resource("")
                    .route(web::post().to(create_guild_command::create_guild_command_handler)),
            )
            .service(
                web::resource("/{guild_id}")
                    .route(web::get().to(get_guild_command::get_guild_command_handler)),
            ),
    );
}
