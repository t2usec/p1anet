use crate::{database::get_db, util::jwt, web::traits::WebError};
use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
    Json,
};
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    return OpenApiRouter::new()
        .routes(routes!(signin))
        .routes(routes!(callback));
}

#[utoipa::path(get, path = "/signin")]
pub async fn signin() -> Redirect {
    let client_id = crate::config::get_config().auth.github.client_id.clone();

    let url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&scope=user:email",
        client_id,
    );
    return Redirect::to(&url);
}

#[utoipa::path(get, path = "/callback")]
pub async fn callback(
    Query(params): Query<crate::web::model::auth::CallbackRequest>,
) -> Result<impl IntoResponse, WebError> {
    let client_id = crate::config::get_config().auth.github.client_id.clone();
    let client_secret = crate::config::get_config()
        .auth
        .github
        .client_secret
        .clone();

    #[derive(Debug, Deserialize, Serialize)]
    struct TokenResponse {
        access_token: String,
    }

    let client = reqwest::Client::new();
    let token_response = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .json(&json!({
            "client_id": client_id,
            "client_secret": client_secret,
            "code": params.code,
        }))
        .send()
        .await?
        .json::<TokenResponse>()
        .await?;

    let access_token = token_response.access_token;

    info!(access_token);

    #[derive(Debug, Deserialize, Serialize)]
    struct GithubUserInfo {
        login: String,
        id: i64,
        avatar_url: String,
        email: Option<String>,
    }

    let client = reqwest::Client::new();

    let user_info = client
        .get("https://api.github.com/user")
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "P1anet")
        .send()
        .await?
        .json::<GithubUserInfo>()
        .await?;

    let mut user = crate::model::user::Entity::find_by_id(user_info.id)
        .one(&get_db())
        .await?;

    let user_active_model = crate::model::user::ActiveModel {
        github_id: Set(user_info.id),
        login: Set(user_info.login),
        avatar_url: Set(user_info.avatar_url),
        last_access_at: Set(chrono::Utc::now().timestamp()),
    };

    if user.is_none() {
        user = Some(user_active_model.insert(&get_db()).await?);
    } else {
        user = Some(user_active_model.update(&get_db()).await?);
    }

    let user = user.unwrap();

    let token = jwt::generate_jwt_token(user.github_id.clone()).await;

    return Ok((
        StatusCode::OK,
        Json(crate::web::model::auth::CallbackResponse {
            code: StatusCode::OK.as_u16(),
            data: user,
            token: token,
        }),
    ));
}
