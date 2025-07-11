use axum::{
    Router, middleware,
    routing::{get, put},
};

use crate::{domains::user::UserRole, infrastructure::middleware::auth::role_check};

use super::handlers::{
    get_me::get_me, get_users::get_users, update_user_password::update_user_password,
    update_user_role::update_user_role, update_username::update_username,
};

pub fn users_handler() -> Router {
    Router::new()
        .route("/me", get(get_me))
        .route(
            "/users",
            get(get_users).layer(middleware::from_fn(|state, req, next| {
                role_check(state, req, next, vec![UserRole::Admin])
            })),
        )
        .route("/name", put(update_username))
        .route("/role", put(update_user_role))
        .route("/password", put(update_user_password))
}
