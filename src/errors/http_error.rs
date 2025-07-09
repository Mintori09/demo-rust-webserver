use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use core::fmt;
use serde::Serialize;
use serde_json::json;
use validator::ValidationErrors;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        Self {
            message: message.into(),
            status,
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        Self::new(message, StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(message, StatusCode::BAD_REQUEST)
    }
    pub fn bad_request_from_validation(errors: &ValidationErrors) -> Self {
        let mut map = serde_json::Map::new();

        for (field, field_errors) in errors.field_errors() {
            if let Some(first) = field_errors.first() {
                let message = first
                    .message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "Invalid value".to_string());

                map.insert(field.to_string(), json!(message));
            }
        }

        let json_str = serde_json::to_string(&json!({
            "status": "fail",
            "errors": map
        }))
        .unwrap_or_else(|_| "Validation failed".to_string());

        Self::new(json_str, StatusCode::BAD_REQUEST)
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        Self::new(message, StatusCode::CONFLICT)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(message, StatusCode::UNAUTHORIZED)
    }

    pub fn into_http_response(self) -> Response {
        let json_response = Json(ErrorResponse {
            status: "fail".to_string(),
            message: self.message.clone(),
        });
        (self.status, json_response).into_response()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: message: {}, status: {}",
            self.message, self.status
        )
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}

impl std::error::Error for HttpError {}
