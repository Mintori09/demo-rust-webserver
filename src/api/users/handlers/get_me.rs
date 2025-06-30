use std::sync::Arc;

use axum::{Extension, Json, response::IntoResponse};

use crate::{
    AppState,
    errors::http_error::HttpError,
    infrastructure::middleware::auth::JWTAuthMiddleware,
    models::user::response::{FilterUser, UserData, UserResponse},
};
pub async fn get_me(
    Extension(_app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
    let filtered_user = FilterUser::filter_user(&user.user);

    let response_data = UserResponse {
        status: "success".to_string(),
        data: UserData {
            user: filtered_user,
        },
    };

    Ok(Json(response_data))
}
