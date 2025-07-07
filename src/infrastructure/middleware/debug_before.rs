use axum::body::to_bytes;
use axum::{body::Body, middleware::Next, response::Response};
use http::Request;
use tracing::{debug, info};

pub async fn debug_before(
    req: Request<Body>,
    next: Next,
) -> Result<Response, crate::errors::http_error::HttpError> {
    let (parts, body) = req.into_parts();

    // Clone body để đọc và tái sử dụng
    let body_bytes = to_bytes(body, usize::MAX).await.unwrap_or_default();
    let body_string = String::from_utf8_lossy(&body_bytes);

    // In thông tin Request
    debug!("->> MIDDLEWARE DEBUG REQUEST");
    info!("Method: {}", parts.method);
    info!("URI: {}", parts.uri);
    info!("Headers:");
    for (key, value) in parts.headers.iter() {
        info!("  {}: {:?}", key, value);
    }
    info!("Body: {}", body_string);

    // Tạo lại request với body cũ (vì body chỉ có thể đọc một lần)
    let req = Request::from_parts(parts, Body::from(body_bytes));

    // Gọi handler tiếp theo
    let response = next.run(req).await;

    Ok(response)
}
