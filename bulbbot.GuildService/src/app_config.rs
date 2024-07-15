use actix_web::web;

use crate::handlers::{guild::*, guild_member::*};

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
            )
            .service(
                web::scope("/member")
                    .service(web::scope("/add").service(web::resource("").route(
                        web::post().to(add_guild_member_command::add_guild_member_command_handler),
                    )))
                    .service(web::scope("/remove").service(
                        web::resource("").route(
                            web::post().to(
                                remove_guild_member_command::remove_guild_member_command_handler,
                            ),
                        ),
                    )),
            ),
    );
}
