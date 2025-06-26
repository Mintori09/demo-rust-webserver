use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domains::user::UserRole;
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
