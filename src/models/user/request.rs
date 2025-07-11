use serde::{Deserialize, Serialize};
use validator::Validate;

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
