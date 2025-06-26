use std::sync::Arc;

use axum::{
    Extension,
    extract::Query,
    http::{HeaderMap, header},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::Cookie;
use validator::Validate;

use crate::{
    AppState,
    domains::user_domain::VerifyEmailQuery,
    errors::{ErrorMessage, HttpError},
    helpers::mail::mails::send_welcome_email,
    infra::user::trait_user::UserExt,
    utils::token,
};

pub async fn verify_email(
    Extension(app_state): Extension<Arc<AppState>>,
    Query(query_params): Query<VerifyEmailQuery>,
) -> Result<impl IntoResponse, HttpError> {
    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;
    let result = app_state
        .db_client
        .get_user(None, None, None, Some(&query_params.token))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = result.ok_or(HttpError::unauthorized(
        ErrorMessage::InvalidToken.to_string(),
    ))?;

    let send_welcome_email_result = send_welcome_email(&user.email, &user.name).await;

    if let Err(e) = send_welcome_email_result {
        eprintln!("Failed to send welcome email: {}", e);
    }

    let token = token::create_token(
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

    let mut headers = HeaderMap::new();

    headers.append(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    let frontend_url = format!("http://local:5173/settings");

    let redirect = Redirect::to(&frontend_url);

    let mut response = redirect.into_response();

    response.headers_mut().extend(headers);
    Ok(response)
}
