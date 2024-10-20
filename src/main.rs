mod config;
mod database;
mod logger;
mod model;
mod selenium;
mod util;
mod web;

use tracing::info;

#[tokio::main]
async fn main() {
    bootstrap().await;
}

async fn bootstrap() {
    logger::init().await;
    config::init().await;
    database::init().await;
    web::init().await;

    let addr = format!(
        "{}:{}",
        config::get_config().axum.host,
        config::get_config().axum.port
    );
    let listener = tokio::net::TcpListener::bind(&addr).await;

    info!("P1anet service has been started at {}.", &addr);

    axum::serve(listener.unwrap(), web::get_app())
        .await
        .unwrap();
}
