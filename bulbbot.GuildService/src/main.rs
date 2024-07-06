mod app_config;
mod database;
mod extractor;
mod handlers;
mod injector;
mod middleware;
mod models;
mod rabbit_mq;
mod schema;

use crate::middleware::telemetry::Telemetry;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use app_config::config_app;
use dotenv::dotenv;
use opentelemetry::global;
use opentelemetry::trace::TraceError;
use std::env;
use tracing::log::info;

#[get("/api/health")]
async fn health() -> impl Responder {
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
        .unwrap_or(String::from("4614"))
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

    let db_pool = database::establish_connection();

    let (_rabbit_mq, rabbit_mq_channel) = rabbit_mq::connect().await;

    info!("Running http server on localhost:{}", server_port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(rabbit_mq_channel.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(Telemetry)
            .configure(config_app)
            .service(health)
    })
    .bind(("127.0.0.1", server_port))
    .unwrap_or_else(|_| panic!("Failed to bind to localhost:{}", server_port))
    .run()
    .await
    .expect("Failed to start server");
}
