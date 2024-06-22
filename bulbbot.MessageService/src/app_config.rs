use actix_web::web;

use crate::handlers::messages;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/messages")
            .service(web::resource("").route(web::post().to(messages::add_message))),
    );
}
