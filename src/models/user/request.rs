use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct RegisterUser {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
    #[validate(
        length(min = 6, message = "Password must be at least 6 characters"),
        must_match(other = "password", message = "Password don't match")
    )]
    pub confirm_password: String,
}

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct RequestQuery {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Debug, Validate, Clone, Serialize, Deserialize)]
pub struct VerifyEmailQuery {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
}

#[derive(Debug, Validate, Clone, Serialize, Deserialize)]
pub struct ForgotPasswordRequest {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
}

#[derive(Debug, Validate, Clone, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,

    #[validate(length(min = 6, message = "New password must be at least 6 characters"))]
    pub new_password: String,

    #[validate(
        length(
            min = 6,
            message = "New password confirm must be at least 6 characters"
        ),
        must_match(other = "new_password", message = "New passwords do not match!")
    )]
    pub new_password_confirm: String,
}
