use chrono::{DateTime, Utc};

pub mod persistent_repo;
pub mod cache_repo;

#[derive(sqlx::FromRow)]
pub struct UserEntity {
    id: i64,
    login: String,
    password_hash: String,
    created_at: DateTime<Utc>
}