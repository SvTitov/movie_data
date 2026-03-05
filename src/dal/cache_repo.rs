use anyhow::{Ok, Result};
use redis::{aio::ConnectionManager, AsyncTypedCommands};

use crate::utils::token::generate_token;

#[derive(Clone)]
pub struct CacheRepo {
    connection: ConnectionManager,
}

impl CacheRepo {
    pub fn new(conn: ConnectionManager) -> Self {
        Self { connection: conn }
    }

    pub async fn session_exists(&mut self, token: &str) -> Result<bool> {
        let key = format!("session:{token}");

        let result = self.connection.exists(key).await?;

        Ok(result)
    }

    pub async fn create_session(&mut self, login: &str) -> Result<String> {
        // One hour
        const LIFE_TIME: u64 = 60 * 60;

        self.revoke_all_sessions(login).await?;

        let user_sess_key = format!("user_sessions:{login}");
        let token = generate_token();

        self.connection.sadd(&user_sess_key, &token).await?;
        self.connection
            .expire(&user_sess_key, LIFE_TIME as i64)
            .await?;

        let key = format!("session:{token}");
        let value = serde_json::json!({
            "created_at": chrono::Utc::now().to_rfc3339()
        })
        .to_string();

        self.connection.set_ex(&key, value, LIFE_TIME).await?;

        Ok(token)
    }

    async fn revoke_all_sessions(&mut self, login: &str) -> Result<()> {
        let usr_sess = format!("user_sessions:{login}");

        let tokens = self.connection.smembers(&usr_sess).await?;

        for token in tokens {
            let session = format!("session:{token}");
            self.connection.del(&session).await?;
        }

        self.connection.del(&usr_sess).await?;

        Ok(())
    }
}

