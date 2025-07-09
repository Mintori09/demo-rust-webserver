use dotenv::dotenv;
use http::HeaderValue;
use std::env;

fn allowed_origins() -> Result<Vec<String>, String> {
    dotenv().ok();

    let raw =
        env::var("ALLOWED_ORIGINS").map_err(|_| "Missing ALLOWED_ORIGINS in .env".to_string())?;

    let list: Vec<String> = raw
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if list.is_empty() {
        return Err("ALLOWED_ORIGINS is empty".to_string());
    }

    Ok(list)
}

pub struct ClientConfig {
    pub allowed_origins: Vec<HeaderValue>,
}

impl ClientConfig {
    pub fn init() -> Self {
        let allowed_origins = allowed_origins().expect("Failed to create client config!");
        let allowed_origins = allowed_origins
            .iter()
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    match HeaderValue::from_str(trimmed) {
                        Ok(val) => Some(val),
                        Err(_) => {
                            eprintln!("⚠️ Invalid origin skipped: {}", trimmed);
                            None
                        }
                    }
                }
            })
            .collect();

        Self { allowed_origins }
    }
}
