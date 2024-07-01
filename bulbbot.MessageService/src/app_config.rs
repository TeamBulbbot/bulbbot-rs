use actix_web::web;

use crate::handlers::*;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/messages")
            .service(web::resource("").route(web::post().to(create_message::create_message))),
    )
    .service(
        web::scope("/api/deletemessages")
            .service(web::resource("").route(web::post().to(delete_message::delete_message))),
    )
    .service(
        web::scope("/api/updatemessage")
            .service(web::resource("").route(web::post().to(update_message::update_message))),
    );
}
