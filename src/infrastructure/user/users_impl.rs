use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    domains::user::{User, UserRole},
    infrastructure::database::database::DBClient,
};

use super::trait_user::UserRepository;

pub struct UserController<'a> {
    pub pool: &'a Pool<Postgres>,
}

impl<'a> UserController<'a> {
    pub fn new(db: &'a DBClient) -> Self {
        Self { pool: &db.pool }
    }
}
#[async_trait]
impl<'a> UserRepository for UserController<'a> {
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
        token: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error> {
        if let Some(id) = user_id {
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(id)
                .fetch_optional(self.pool)
                .await
        } else if let Some(n) = name {
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = $1")
                .bind(n)
                .fetch_optional(self.pool)
                .await
        } else if let Some(e) = email {
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
                .bind(e)
                .fetch_optional(self.pool)
                .await
        } else if let Some(t) = token {
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE verification_token = $1")
                .bind(t)
                .fetch_optional(self.pool)
                .await
        } else {
            Ok(None)
        }
    }
    async fn get_users(&self, page: u32, limit: usize) -> Result<Vec<User>, sqlx::Error> {
        let offset = (page.saturating_sub(1) as usize) * limit;

        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(self.pool)
        .await?;

        Ok(users)
    }
    async fn save_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
        verification_token: T,
        token_expires_at: DateTime<Utc>,
    ) -> Result<User, sqlx::Error> {
        let name = name.into();
        let email = email.into();
        let password = password.into();
        let verification_token = verification_token.into();

        let user = sqlx::query_as::<_, User>(
            r#" INSERT INTO users ( id, name, email, password, role, verified, 
                verification_token, token_expires_at, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(name)
        .bind(email)
        .bind(password)
        .bind(UserRole::User)
        .bind(false)
        .bind(verification_token)
        .bind(token_expires_at)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(self.pool)
        .await?;

        Ok(user)
    }
    async fn get_user_count(&self) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(self.pool)
            .await?;
        Ok(row.0)
    }

    async fn update_username<T: Into<String> + Send>(
        &self,
        user_id: Uuid,
        name: T,
    ) -> Result<User, sqlx::Error> {
        let name = name.into();
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET name = $1, updated_at = $2 WHERE id = $3 RETURNING *",
        )
        .bind(name)
        .bind(Utc::now())
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;
        Ok(user)
    }

    async fn update_user_role(&self, user_id: Uuid, role: UserRole) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET role = $1, updated_at = $2 WHERE id = $3 RETURNING *",
        )
        .bind(role)
        .bind(Utc::now())
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;
        Ok(user)
    }

    async fn update_user_password(
        &self,
        user_id: Uuid,
        password: String,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET password = $1, updated_at = $2 WHERE id::text = $3 RETURNING *",
        )
        .bind(password)
        .bind(Utc::now())
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;
        Ok(user)
    }

    async fn verified_token(&self, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET verified = true, updated_at = $1 WHERE verification_token = $2",
        )
        .bind(Utc::now())
        .bind(token)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    async fn add_verified_token(
        &self,
        user_id: Uuid,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET verification_token = $1, token_expires_at = $2, updated_at = $3 WHERE id = $4",
        )
        .bind(token)
        .bind(expires_at)
        .bind(Utc::now())
        .bind(user_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
