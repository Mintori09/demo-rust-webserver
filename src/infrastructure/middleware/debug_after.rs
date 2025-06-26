use axum::{
    body::{Body, to_bytes},
    middleware::Next,
    response::Response,
};
use http::Request;
use tracing::{debug, info};

use crate::errors::http_error::HttpError;

pub async fn debug_after(req: Request<Body>, next: Next) -> Result<Response, HttpError> {
    // Chạy handler để lấy response
    let response = next.run(req).await;

    let status = response.status();
    let headers = response.headers().clone();

    // Tách body để đọc
    let (parts, body) = response.into_parts();
    let body_bytes = to_bytes(body, 1024 * 1024).await.unwrap_or_default(); // giới hạn 1MB
    let body_string = String::from_utf8_lossy(&body_bytes);
    let preview = &body_string[..body_string.len().min(1024)];

    debug!("->> MIDDLEWARE DEBUG RESPONSE");
    info!("Status: {}", status);
    info!("Headers:");
    for (key, value) in headers.iter() {
        info!("  {}: {:?}", key, value);
    }
    info!("Body (first 1KB): {}", preview);

    // Trả lại response với body cũ
    Ok(Response::from_parts(parts, Body::from(body_bytes)))
}
