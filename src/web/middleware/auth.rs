use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sea_orm::EntityTrait;
use serde_json::json;

use crate::{
    database::get_db,
    util,
    web::traits::{Ext, WebError},
};

pub async fn jwt(mut req: Request<Body>, next: Next) -> Result<Response, WebError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .unwrap_or("");

    let decoding_key = DecodingKey::from_secret(util::jwt::get_secret().await.as_bytes());
    let validation = Validation::default();

    let result = decode::<util::jwt::Claims>(token, &decoding_key, &validation);

    if let Ok(token_data) = result {
        let result = crate::model::user::Entity::find_by_id(token_data.claims.github_id)
            .one(&get_db())
            .await;

        if let Err(_err) = result {
            return Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    "msg": "internal_server_error"
                })),
            )
                .into_response());
        }

        let user = result.unwrap();

        if user.is_none() {
            return Err(WebError::NotFound(String::from("not_found")));
        }

        let user = user.unwrap();

        req.extensions_mut().insert(Ext {
            operator: Some(user.clone()),
        });
    }

    return Ok(next.run(req).await);
}
