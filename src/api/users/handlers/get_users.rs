use std::sync::Arc;

use axum::{Extension, Json, extract::Query, response::IntoResponse};
use validator::Validate;

use crate::{
    AppState,
    errors::http_error::HttpError,
    infrastructure::user::trait_user::UserExt,
    models::user::{
        request::RequestQuery,
        response::{FilterUser, UserListResponse},
    },
};
pub async fn get_users(
    Query(query_params): Query<RequestQuery>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let users = app_state
        .db_client
        .get_users(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user_count = app_state
        .db_client
        .get_user_count()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = UserListResponse {
        status: "success".to_string(),
        users: FilterUser::filter_users(&users),
        results: user_count,
    };

    Ok(Json(response))
}
