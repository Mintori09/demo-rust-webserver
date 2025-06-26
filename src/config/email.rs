use std::env;

use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct MySMTP {
    pub username: String,
    pub password: String,
    pub server: String,
    pub port: u16,
}

impl MySMTP {
    pub fn init() -> Self {
        dotenv().ok();

        Self {
            username: env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set"),
            password: env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set"),
            server: env::var("SMTP_SERVER").expect("SMTP_SERVER must be set"),
            port: env::var("SMTP_PORT")
                .expect("SMTP_PORT must be set")
                .parse()
                .expect("SMTP_PORT must be a valid number"),
        }
    }
}
