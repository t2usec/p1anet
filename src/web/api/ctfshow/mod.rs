use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use utoipa::OpenApi;

use crate::{
    database::get_db,
    web::{
        model::ctfshow::{GetCookiesResponse, GetStatusResponse},
        traits::{Ext, WebError},
    },
};

#[derive(OpenApi)]
#[openapi(paths(get_cookies, get_status))]
pub struct Doc;

pub fn router() -> Router {
    return Router::new()
        .route("/", get(get_cookies))
        .route("/status", get(get_status));
}

#[utoipa::path(get, path = "")]
pub async fn get_cookies(Extension(ext): Extension<Ext>) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;

    let mut latest_obtained_at: i64 = 0;

    let latest_record = crate::model::record::Entity::find()
        .order_by_desc(crate::model::record::Column::ObtainedAt)
        .one(&get_db())
        .await?;

    if let Some(latest_record) = latest_record {
        latest_obtained_at = latest_record.obtained_at;
    }

    let self_related_records = crate::model::record::Entity::find()
        .filter(crate::model::record::Column::UserId.eq(operator.github_id))
        .filter(
            crate::model::record::Column::ObtainedAt
                .gt(chrono::Utc::now().timestamp() - 24 * 60 * 60),
        )
        .all(&get_db())
        .await?;

    if self_related_records.len() >= 2 {
        return Err(WebError::BadRequest(String::from("too_many_times")));
    }

    if latest_obtained_at + 3 * 60 * 60 > chrono::Utc::now().timestamp() {
        return Err(WebError::BadRequest(String::from("time_less_than_3_hours")));
    }

    let cookies = crate::selenium::get_ctfshow_cookies().await?;

    let record = crate::model::record::ActiveModel {
        user_id: Set(operator.github_id),
        obtained_at: Set(chrono::Utc::now().timestamp()),
        ..Default::default()
    };

    record.insert(&get_db()).await?;

    return Ok((
        StatusCode::OK,
        Json(GetCookiesResponse {
            code: StatusCode::OK.as_u16(),
            data: cookies,
        }),
    ));
}

#[utoipa::path(get, path = "/status")]
pub async fn get_status(Extension(ext): Extension<Ext>) -> Result<impl IntoResponse, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;

    let latest_record_with_user = crate::model::record::Entity::find()
        .find_also_related(crate::model::user::Entity)
        .order_by_desc(crate::model::record::Column::ObtainedAt)
        .one(&get_db())
        .await?;

    if latest_record_with_user.clone().is_none() {
        return Ok((
            StatusCode::OK,
            Json(GetStatusResponse {
                code: StatusCode::OK.as_u16(),
                data: None,
            }),
        ));
    }

    let mut latest_record = latest_record_with_user.clone().unwrap().0;
    let user = latest_record_with_user.unwrap().1;
    latest_record.user = user;

    return Ok((
        StatusCode::OK,
        Json(GetStatusResponse {
            code: StatusCode::OK.as_u16(),
            data: Some(latest_record),
        }),
    ));
}
