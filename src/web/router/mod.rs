pub mod auth;
pub mod ctfshow;

use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;
use utoipa_axum::{router::OpenApiRouter, routes};

pub async fn router() -> OpenApiRouter {
    return OpenApiRouter::new()
        .routes(routes!(index))
        .routes(routes!(get_github_client_id))
        .nest("/auth", auth::router())
        .nest("/ctfshow", ctfshow::router());
}

#[utoipa::path(get, path = "")]
pub async fn index() -> impl IntoResponse {
    return Json(json!({
        "code": StatusCode::OK.as_u16(),
    }));
}

#[utoipa::path(get, path = "/github")]
pub async fn get_github_client_id() -> impl IntoResponse {
    return Json(json!({
        "code": StatusCode::OK.as_u16(),
        "data": crate::config::get_config().auth.github.client_id,
    }));
}
