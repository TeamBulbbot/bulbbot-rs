mod app_config;
mod database;
mod handlers;
mod models;
mod schema;

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use app_config::config_app;
use dotenv::dotenv;
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

    info!("Running http server on localhost:{}", server_port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(config_app)
            .service(health)
    })
    .bind(("127.0.0.1", server_port))
    .expect(&format!("Failed to bind to localhost:{}", server_port))
    .run()
    .await
    .expect("Failed to start server");
}
