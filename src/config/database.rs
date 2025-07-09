use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port: u16,
}

impl Config {
    pub fn init() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

        let jwt_secret = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set!");

        let jwt_maxage = env::var("JWT_MAXAGE")
            .expect("JWT_MAXAGE must be set!")
            .parse::<i64>()
            .expect("JWT_MAXAGE must be a valid i64 number");

        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(8000);

        Self {
            database_url,
            jwt_secret,
            jwt_maxage,
            port,
        }
    }
}
