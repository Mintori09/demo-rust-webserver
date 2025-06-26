use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::user::{User, UserRole};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUser {
    pub fn filter_user(user: &User) -> Self {
        Self {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            verified: user.verified,
            role: user.role.to_str().to_string(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
    pub fn filter_users(user: &[User]) -> Vec<Self> {
        user.iter().map(FilterUser::filter_user).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponse {
    pub status: String,
    pub users: Vec<FilterUser>,
    pub results: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponse {
    pub status: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct NameUpdate {
    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,
}

#[derive(Debug, Validate, Clone, Serialize, Deserialize)]
pub struct RoleUpdate {
    #[validate(custom(function = "validate_user_role"))]
    pub role: UserRole,
}

fn validate_user_role(role: &UserRole) -> Result<(), validator::ValidationError> {
    if matches!(role, UserRole::User | UserRole::Admin) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_role"))
    }
}

#[derive(Debug, Validate, Clone, Serialize, Deserialize)]
pub struct UserPasswordUpdate {
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
    #[validate(length(min = 6, message = "Old password must be at least 6 characters"))]
    pub old_password: String,
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
