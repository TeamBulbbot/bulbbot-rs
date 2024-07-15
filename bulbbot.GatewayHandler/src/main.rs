mod events;
mod handler;
mod models;
mod rabbit_mq;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use common::telemetry::extractor_rabbitmq::RabbitMqExtractor;
use dotenv::dotenv;
use handler::Handler;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions},
    types::FieldTable,
};
use opentelemetry::global::{self};
use opentelemetry::trace::{Status, TraceContextExt, TraceError};
use serenity::futures::StreamExt;
use std::env;
use std::str;
use tracing::log::info;

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

    let (rabbit_mq, rabbit_mq_channel) = rabbit_mq::connect().await;

    let handler = Handler::init();

    let mut consumer = rabbit_mq_channel
        .basic_consume(
            "bulbbot.gateway",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    tokio::spawn(async move {
        info!("Rabbit MQ Consumer started");
        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.expect("Error trying to consume");
            let event_data =
                str::from_utf8(&delivery.data).expect("Failed to convert binary to utf8");

            let headers = delivery.properties.headers().clone().unwrap_or_default();
            let cx = global::get_text_map_propagator(|propagator| {
                propagator.extract(&RabbitMqExtractor(&headers))
            });

            let response = handler.handle(event_data, &cx).await;
            match response {
                true => {
                    delivery
                        .ack(BasicAckOptions::default())
                        .await
                        .expect("failed to ack request");
                    cx.span().set_status(Status::Ok);
                }
                false => {
                    delivery
                        .nack(BasicNackOptions::default())
                        .await
                        .expect("failed to nack request");
                    cx.span().set_status(Status::Error {
                        description: String::from("Request failed").into(),
                    })
                }
            };

            cx.span().end();
        }
    });

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");

        rabbit_mq_channel
            .close(200, "Normal shutdown")
            .await
            .expect("Failed to close Rabbit MQ channel");
        rabbit_mq
            .close(200, "Normal shutdown")
            .await
            .expect("Failed to close Rabbit MQ connection");
    });

    info!("Running http server on localhost:{}", server_port);
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", server_port))
        .expect("Failed to bind to localhost:8080")
        .run()
        .await
        .expect("Failed to start server");
}
