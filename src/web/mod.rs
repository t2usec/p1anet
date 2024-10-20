pub mod docs;
pub mod middleware;
pub mod model;
pub mod router;
pub mod traits;

use docs::ApiDoc;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use std::sync::OnceLock;

use axum::{middleware::from_fn, Router};
use reqwest::Method;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

static APP: OnceLock<Router> = OnceLock::new();

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

    let (app, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(
            OpenApiRouter::new()
                .nest("/api", router::router().await)
                .layer(from_fn(middleware::auth::jwt))
                .layer(TraceLayer::new_for_http()),
        )
        .layer(from_fn(middleware::frontend::serve))
        .layer(cors)
        .split_for_parts();

    let x = api.clone().to_json().unwrap();

    APP.set(app.route(
        "/openapi.json",
        axum::routing::get(move || async { return x }),
    ))
    .unwrap();
}

pub fn get_app() -> Router {
    return APP.get().unwrap().clone();
}
