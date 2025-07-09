use api::router::create_router;
use axum::{
    Extension, Router,
    http::{
        Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
    middleware,
};
use config::{client::ClientConfig, database::Config};
use infrastructure::{
    database::database::DBClient,
    middleware::{debug_after::debug_after, debug_before::debug_before},
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;

pub mod api;
pub mod config;
pub mod core;
pub mod domains;
pub mod errors;
pub mod helpers;
pub mod infrastructure;
pub mod models;
pub mod utils;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

pub async fn run() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let config = Config::init();
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {}", err);
            std::process::exit(1);
        }
    };
    let cors = CorsLayer::new()
        .allow_origin(ClientConfig::init().allowed_origins)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT]);
    let db_client = DBClient::new(pool);
    let app_state = Arc::new(AppState {
        env: config.clone(),
        db_client,
    });

    let app = Router::new()
        .merge(create_router(app_state.clone()))
        .layer(middleware::from_fn(debug_after))
        .layer(middleware::from_fn(debug_before))
        .layer(Extension(app_state))
        .layer(cors.clone());

    println!("Server is running on http://localhost:{}", config.port);
    println!("{}", config.port);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
