use std::sync::Arc;

use axum::{Extension, Json, response::IntoResponse};
use chrono::{Duration, Utc};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    errors::http_error::HttpError,
    helpers::mail::mails::send_forgot_password_email,
    infrastructure::user::{user_trait::UserRepository, users_impl::PgUserRepository},
    models::user::response::Response,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Validate, Clone, Serialize, Deserialize)]
pub struct ForgotPasswordRequest {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
}

pub async fn forgot_password(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<ForgotPasswordRequest>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let result = PgUserRepository::new(&app_state.db_client)
        .get_user(None, None, Some(&body.email), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = result.ok_or(HttpError::bad_request("Email not found!".to_string()))?;

    let verification_token = Uuid::new_v4().to_string();

    let expires_at = Utc::now() + Duration::hours(24);

    let user_id = Uuid::parse_str(&user.id.to_string()).unwrap();

    PgUserRepository::new(&app_state.db_client)
        .add_verified_token(user_id, &verification_token, expires_at)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let rest_link = format!(
        "http://localhost:5173/reset-password?token={}",
        &verification_token
    );

    let email_sent = send_forgot_password_email(&user.email, &rest_link, &user.name).await;

    if let Err(e) = email_sent {
        eprintln!("Failed to send forgot password mail: {}", e);
    }

    Ok(Json(Response {
        message: "Password reset link has been sent to your email!".to_string(),
        status: "success",
    }))
}
