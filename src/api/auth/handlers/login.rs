use std::sync::Arc;

use axum::{Extension, Json, http::header, response::IntoResponse};
use cookie::Cookie;
use validator::Validate;

use crate::{
    AppState,
    errors::{error_message::ErrorMessage, http_error::HttpError},
    infrastructure::user::{trait_user::UserRepository, users_impl::UserController},
    models::user::{request::LoginUser, response::UserLoginResponse},
    utils::{password, token},
};

pub async fn login(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<LoginUser>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = UserController::new(&app_state.db_client)
        .get_user(None, None, Some(&body.email), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = result.ok_or(HttpError::bad_request(
        ErrorMessage::WrongCredentials.to_string(),
    ))?;

    let password_matched = password::compare(&body.password, &user.password)
        .map_err(|_| HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))?;

    if password_matched {
        let token = token::generate_token(
            &user.id.to_string(),
            &app_state.env.jwt_secret.as_bytes(),
            app_state.env.jwt_maxage,
        )
        .map_err(|e| HttpError::server_error(e.to_string()))?;

        let cookie_duration = time::Duration::minutes(app_state.env.jwt_maxage * 60);

        let cookie = Cookie::build(("token", token.clone()))
            .path("/")
            .max_age(cookie_duration)
            .http_only(true)
            .build();

        let reponse = Json(UserLoginResponse {
            status: "success".to_string(),
            token,
        });
        let mut response = reponse.into_response();
        response
            .headers_mut()
            .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
        Ok(response)
    } else {
        Err(HttpError::bad_request(
            ErrorMessage::WrongCredentials.to_string(),
        ))
    }
}
