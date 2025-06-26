use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use chrono::{Duration, Utc};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    domains::user_domain::{RegisterUser, Response},
    errors::{ErrorMessage, HttpError},
    helpers::mail::mails::send_verification_email,
    infra::user::trait_user::UserExt,
    utils::password,
};

pub async fn register(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<RegisterUser>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let verification_token = Uuid::new_v4().to_string();
    let expires_at: chrono::DateTime<Utc> = Utc::now() + Duration::hours(24);

    let hash_password = password::hash_password(&body.password)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let result = app_state
        .db_client
        .save_user(
            &body.name,
            &body.email,
            &hash_password,
            &verification_token,
            expires_at,
        )
        .await;

    match result {
        Ok(_user) => {
            let send_email_result =
                send_verification_email(&body.email, &body.name, &verification_token).await;
            if let Err(e) = send_email_result {
                eprintln!("Failed to send verification email: {}", e);
            }
            Ok((
                StatusCode::CREATED,
                Json(Response {
                    status: "success",
                    message:
                        "registration successful! Please check your email to verifu your account."
                            .to_string(),
                }),
            ))
        }
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(
                    ErrorMessage::EmailExist.to_string(),
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}
