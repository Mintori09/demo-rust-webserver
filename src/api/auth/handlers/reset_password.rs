use std::sync::Arc;

use axum::{Extension, Json, response::IntoResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    errors::http_error::HttpError,
    infrastructure::user::{user_trait::UserRepository, users_impl::PgUserRepository},
    models::user::response::Response,
    utils::password,
};

#[derive(Debug, Validate, Clone, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,

    #[validate(length(min = 6, message = "New password must be at least 6 characters"))]
    pub new_password: String,

    #[validate(
        length(
            min = 6,
            message = "New password confirm must be at least 6 characters"
        ),
        must_match(other = "new_password", message = "New passwords do not match!")
    )]
    pub new_password_confirm: String,
}

pub async fn reset_password(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user_controller = PgUserRepository::new(&app_state.db_client);
    let result = PgUserRepository::new(&app_state.db_client)
        .get_user(None, None, None, Some(&body.token))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    let user = result.ok_or(HttpError::bad_request(
        "Invalid or expired token".to_string(),
    ))?;

    if let Some(expires_at) = user.token_expires_at {
        if Utc::now() > expires_at {
            return Err(HttpError::bad_request(
                "Verification token has expired.".to_string(),
            ));
        } else {
            return Err(HttpError::bad_request(
                "Invalid verification token.".to_string(),
            ));
        }
    }

    let user_id = Uuid::parse_str(&user.id.to_string()).unwrap();

    let hash_password = password::hash_password(&body.new_password)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    user_controller
        .update_user_password(user_id.clone(), hash_password)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    user_controller
        .verified_token(&body.token)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = Response {
        status: "success",
        message: "Password has been successfully reset.".to_string(),
    };

    Ok(Json::from(response).into_response())
}
