pub mod auth;
pub mod ctfshow;

use axum::{response::IntoResponse, routing::get, Json, Router};
use reqwest::StatusCode;
use serde_json::json;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(index, get_github_client_id),
    nest(
        (path = "/auth", api = auth::Doc),
        (path = "/ctfshow", api = ctfshow::Doc)
    ),
)]
pub struct Doc;

pub async fn router() -> Router {
    return Router::new()
        .route("/", get(index))
        .route("/github", get(get_github_client_id))
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
