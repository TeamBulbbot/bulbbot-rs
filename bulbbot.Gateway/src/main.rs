mod events;
mod manger_container_structs;
mod rabbit_mq;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use events::event_handler::Handler;
use manger_container_structs::{RabbitMQMangerContainer, ShardManagerContainer};
use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::{global, trace::TraceError};
use serenity::prelude::*;
use std::env;
use tracing::log::{error, info};

#[get("/health")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Healthy!")
}

fn init_tracer_provider() -> Result<opentelemetry_sdk::trace::Tracer, TraceError> {
    global::set_text_map_propagator(opentelemetry_zipkin::Propagator::new());
    opentelemetry_zipkin::new_pipeline()
        .with_service_name(format!(
            "{}-{}-{}",
            env::var("ENVIRONMENT").expect("[ENV] expected 'ENVIRONMENT' in the environment"),
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .with_collector_endpoint(
            env::var("ZIPKIN_URL").expect("[ENV] expected 'ZIPKIN_URL' in the environment"),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let tracer_provider = init_tracer_provider().expect("Failed to init tracer");
    global::set_tracer_provider(tracer_provider.provider().unwrap().clone());

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

    let mut client = Client::builder(
        env::var("DISCORD_TOKEN").expect("[ENV] expected 'DISCORD_TOKEN' in the environment"),
        intents,
    )
    .event_handler(Handler)
    .await
    .expect("[STARTUP] error creating client");

    let (rabbit_mq, rabbit_mq_channel) = rabbit_mq::connect().await;

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<RabbitMQMangerContainer>(rabbit_mq_channel.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");

        shard_manager.shutdown_all().await;

        rabbit_mq_channel
            .close(200, "Normal shutdown")
            .await
            .expect("Failed to close Rabbit MQ channel");
        rabbit_mq
            .close(200, "Normal shutdown")
            .await
            .expect("Failed to close Rabbit MQ connection");
        shutdown_tracer_provider();
    });

    tokio::spawn(async move {
        info!("Running http server on localhost:{}", server_port);
        HttpServer::new(|| App::new().service(hello))
            .bind(("127.0.0.1", server_port))
            .unwrap_or_else(|_| panic!("Failed to bind to localhost:8080"))
            .run()
            .await
            .expect("Failed to start server");
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:#?}", why);
    }
}
