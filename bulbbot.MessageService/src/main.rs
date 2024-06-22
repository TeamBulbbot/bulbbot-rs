mod app_config;
mod handlers;
mod models;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use app_config::config_app;
use dotenv::dotenv;
use serenity::prelude::*;
use std::env;
use tracing::log::info;

#[get("/api/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Healthy!")
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let server_port = env::var("SERVER_PORT")
        .unwrap_or(String::from("8080"))
        .parse::<u16>()
        .expect("Invalid server port");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    info!(
        "{} on version: {} - {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_REPOSITORY")
    );

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::GUILD_INVITES
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_SCHEDULED_EVENTS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(
        env::var("DISCORD_TOKEN").expect("[ENV] expected 'DISCORD_TOKEN' in the environment"),
        intents,
    )
    .await
    .expect("[STARTUP] error creating client");

    info!("Running http server on localhost:{}", server_port);
    HttpServer::new(|| App::new().configure(config_app).service(health))
        .bind(("127.0.0.1", server_port))
        .expect(&format!("Failed to bind to localhost:{}", server_port))
        .run()
        .await
        .expect("Failed to start server");
}
