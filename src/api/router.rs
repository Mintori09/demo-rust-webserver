use std::sync::Arc;

use axum::{Extension, Router, middleware};
use tower_http::trace::TraceLayer;

use crate::{AppState, infrastructure::middleware::auth::auth};

use super::{auth::routes::auth_handler, users::routes::users_handler};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_route = Router::new()
        .nest("/auth", auth_handler())
        .nest("/users", users_handler().layer(middleware::from_fn(auth)))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state));

    Router::new().nest("/api", api_route)
}
