use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domains::user::User;

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
