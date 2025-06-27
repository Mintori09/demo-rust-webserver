use axum::{
    Extension,
    extract::Query,
    http::header,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::cookie::Cookie;
use std::sync::Arc;
use validator::Validate;

use crate::{
    AppState,
    domains::user::User,
    errors::{error_message::ErrorMessage, http_error::HttpError},
    helpers::mail::mails::send_welcome_email,
    infrastructure::user::trait_user::UserExt,
    models::user::request::VerifyEmailQuery,
    utils::token,
};

pub async fn verify_email(
    Extension(app_state): Extension<Arc<AppState>>,
    Query(query_params): Query<VerifyEmailQuery>,
) -> Result<impl IntoResponse, HttpError> {
    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = get_user_by_token(&app_state, &query_params.token).await?;

    if !user.verified {
        verify_user_token(&app_state, &query_params.token).await?;
        try_send_welcome_email(&user.email, &user.name).await;
    }

    let jwt_cookie = generate_jwt_cookie(&app_state, &user.id.to_string())?;
    let response = redirect_with_cookie("http://localhost:5173/settings", jwt_cookie);

    Ok(response)
}

async fn get_user_by_token(app_state: &Arc<AppState>, token: &str) -> Result<User, HttpError> {
    app_state
        .db_client
        .get_user(None, None, None, Some(token))
        .await
        .map_err(server_error)?
        .ok_or_else(|| HttpError::unauthorized(ErrorMessage::InvalidToken.to_string()))
}

async fn verify_user_token(app_state: &Arc<AppState>, token: &str) -> Result<(), HttpError> {
    app_state
        .db_client
        .verified_token(token)
        .await
        .map_err(server_error)
}

async fn try_send_welcome_email(email: &str, name: &str) {
    if let Err(e) = send_welcome_email(email, name).await {
        eprintln!("Failed to send welcome email: {}", e);
    }
}

fn generate_jwt_cookie(
    app_state: &Arc<AppState>,
    user_id: &str,
) -> Result<Cookie<'static>, HttpError> {
    let jwt = token::generate_token(
        user_id,
        app_state.env.jwt_secret.as_bytes(),
        app_state.env.jwt_maxage,
    )
    .map_err(server_error)?;

    Ok(Cookie::build(("token", jwt))
        .path("/")
        .max_age(time::Duration::minutes(app_state.env.jwt_maxage * 60))
        .http_only(true)
        .build())
}

fn redirect_with_cookie(location: &str, cookie: Cookie) -> Response {
    let mut response = Redirect::to(location).into_response();
    response
        .headers_mut()
        .append(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    response
}

fn server_error<E: ToString>(err: E) -> HttpError {
    HttpError::server_error(err.to_string())
}
