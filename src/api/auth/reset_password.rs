use std::sync::Arc;

use axum::{Extension, Json, response::IntoResponse};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    domains::user_domain::{ResetPasswordRequest, Response},
    errors::HttpError,
    infra::user::trait_user::UserExt,
    utils::password,
};

pub async fn reset_password(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
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

    app_state
        .db_client
        .update_user_password(user_id.clone(), hash_password)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    app_state
        .db_client
        .verified_token(&body.token)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = Response {
        message: "Password has been successfully reset.".to_string(),
        status: "success",
    };

    Ok(Json(response).into_response())
}
