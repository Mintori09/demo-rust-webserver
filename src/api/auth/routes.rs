use axum::{
    Router,
    routing::{get, post},
};

use super::handlers::{
    forgot_password::forgot_password, login::login, register::register,
    reset_password::reset_password, verify_email::verify_email,
};

pub fn auth_handler() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
        .route("/verify-email", get(verify_email))
}
