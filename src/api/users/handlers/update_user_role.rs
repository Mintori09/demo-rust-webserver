use std::sync::Arc;

use axum::{Extension, Json, response::IntoResponse};
use validator::Validate;

use crate::{
    AppState,
    errors::http_error::HttpError,
    infrastructure::{middleware::auth::JWTAuthMiddleware, user::trait_user::UserRepository},
    models::user::{
        response::{FilterUser, UserData, UserResponse},
        update::RoleUpdate,
    },
};
pub async fn update_user_role(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddleware>,
    Json(body): Json<RoleUpdate>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = &user.user;

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    let result = app_state
        .db_client
        .update_user_role(user_id.clone(), body.role)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let filtered_user = FilterUser::filter_user(&result);

    let response = UserResponse {
        data: UserData {
            user: filtered_user,
        },
        status: "success".to_string(),
    };

    Ok(Json(response))
}
