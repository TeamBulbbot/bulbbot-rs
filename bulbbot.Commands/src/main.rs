mod types;
mod verify;

use crate::{
    types::interaction::{
        Interaction, InteractionResponse, InteractionResponseType, InteractionType,
    },
    verify::DiscordVerify,
};
use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

fn get_header<'a>(name: &'a str, request: &'a HttpRequest) -> Option<&'a str> {
    request.headers().get(name)?.to_str().ok()
}

fn verify_discord_message(request: &HttpRequest, req_body: &String) -> bool {
    let verify = DiscordVerify {
        public_key: String::from(
            env::var("PUBLIC_KEY").expect("Missing 'PUBLIC_KEY' from environment"),
        ),
        signature: get_header("x-signature-ed25519", &request)
            .unwrap_or_else(|| "")
            .to_string(),
        timestamp: get_header("x-signature-timestamp", &request)
            .unwrap_or_else(|| "")
            .to_string(),
    };

    verify.verify_request(&req_body)
}

#[post("/")]
async fn interaction(request: HttpRequest, req_body: String) -> impl Responder {
    if !verify_discord_message(&request, &req_body) {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    let interaction =
        serde_json::from_str::<Interaction>(&req_body).expect("Failed to unwrap to interaction");

    if interaction.interaction_type == InteractionType::Ping {
        return HttpResponse::Ok().json(web::Json(InteractionResponse {
            data: None,
            interaction_type: InteractionResponseType::Pong,
        }));
    }

    if interaction.interaction_type == InteractionType::ApplicationCommand {
        let interaction_data = match interaction.data {
            Some(data) => data,
            None => return HttpResponse::BadRequest().body("Bad request"),
        };
    }

    HttpResponse::Ok().json(web::Json(InteractionResponse {
        data: None,
        interaction_type: InteractionResponseType::Pong,
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| App::new().service(interaction))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
