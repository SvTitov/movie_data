use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};

use crate::{dal::UserEntity, utils::encryption};

#[async_trait]
pub trait PersistentRepo: Send + Sync {
    async fn create_user(
        &self,
        login: &str,
        password: &str,
        created_at: DateTime<Utc>,
    ) -> Result<()>;
    async fn get_all_users(&self) -> Result<Vec<UserEntity>>;
    async fn user_valid(&self, login: &str, password: &str) -> Result<bool>;
}

#[derive(Clone)]
pub struct RealPersistentRepo {
    pool: Pool<Postgres>,
}

impl RealPersistentRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PersistentRepo for RealPersistentRepo {
    async fn create_user(
        &self,
        login: &str,
        password: &str,
        created_at: DateTime<Utc>,
    ) -> Result<()> {
        let pass_hash = encryption::encrypt_password(password)?;

        sqlx::query("INSERT INTO users (login, password_hash, created_at) VALUES ($1, $2, $3)")
            .bind(login)
            .bind(pass_hash)
            .bind(created_at)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_all_users(&self) -> Result<Vec<UserEntity>> {
        let users: Vec<UserEntity> =
            sqlx::query_as("SELECT id, login, password_hash, created_at FROM users ORDER BY id")
                .fetch_all(&self.pool)
                .await?;

        Ok(users)
    }

    async fn user_valid(&self, login: &str, password: &str) -> Result<bool> {
        let pass_hash: Option<String> =
            sqlx::query_scalar("SELECT password_hash FROM users WHERE login = $1")
                .bind(login)
                .fetch_one(&self.pool)
                .await?;

        if let Some(pass_hash) = pass_hash {
            let is_valid = encryption::veryfy_password(password, &pass_hash);

            return Ok(is_valid);
        }

        Err(anyhow!("No user found!"))
    }
}

