pub mod api;
pub mod middleware;
pub mod model;
pub mod traits;

use std::sync::OnceLock;

use axum::{middleware::from_fn, Json, Router};
use reqwest::Method;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use utoipa::OpenApi;

static APP: OnceLock<Router> = OnceLock::new();

#[derive(OpenApi)]
#[openapi(
    info(title = "P1anet API", description = "OpenAPI docs for P1anet API."),
    nest(
        (path = "/api", api = api::Doc)
    ),
)]
pub struct Doc;

pub async fn init() {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .merge(
            Router::new()
                .nest("/api", api::router().await)
                .layer(from_fn(middleware::auth::jwt))
                .layer(TraceLayer::new_for_http()),
        )
        .layer(from_fn(middleware::frontend::serve))
        .layer(cors)
        .route(
            "/openapi.json",
            axum::routing::get(move || async move { Json(Doc::openapi()) }),
        );

    APP.set(app).unwrap();
}

pub fn get_app() -> Router {
    return APP.get().unwrap().clone();
}
