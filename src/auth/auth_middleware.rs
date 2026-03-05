use anyhow::Result;
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use redis::TypedCommands;
use thiserror::Error;

use crate::AppState;

#[derive(Debug, Error)]
enum TokenValidationError {
    #[error("The token has been expired.")]
    ExpiredToken,

    #[error("No valid token")]
    WrongToken,
}

pub(super) async fn auth_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|x| x.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let is_valid = is_valid_token(auth)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if is_valid {
        return Ok(next.run(req).await);
    }

    Err(StatusCode::UNAUTHORIZED)
}

async fn is_valid_token(token: &str) -> Result<bool> {
    let clean_token = token
        .strip_prefix("Bearer ")
        .ok_or(TokenValidationError::ExpiredToken)?;
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    let mut r_connection = client.get_connection()?;

    let session = format!("session:{clean_token}");

    Ok(r_connection.exists(session)?)
}

mod test {

    use anyhow::Result;
    use async_trait::async_trait;

    use axum::{middleware, routing::get, Router};
    use axum_test::TestServer;
    use chrono::{DateTime, Utc};
    use redis::TypedCommands;

    use crate::{
        auth::auth_middleware::auth_middleware,
        dal::{cache_repo::CacheRepo, persistent_repo::PersistentRepo, UserEntity},
        AppState,
    };

    struct MockDataBase;
    #[async_trait]
    impl PersistentRepo for MockDataBase {
        async fn create_user(
            &self,
            login: &str,
            password: &str,
            created_at: DateTime<Utc>,
        ) -> Result<()> {
            todo!()
        }
        async fn get_all_users(&self) -> Result<Vec<UserEntity>> {
            todo!()
        }
        async fn user_valid(&self, login: &str, password: &str) -> Result<bool> {
            todo!()
        }
    }

    #[tokio::test]
    async fn should_validate_token_correctly() {
        // Arrange
        let mut client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut connection = client.get_connection().unwrap();
        let c_m = client.get_connection_manager().await.unwrap();

        connection
            .set("session:123", "{ \"login\": \"test_urs\" }")
            .unwrap();

        let cache = CacheRepo::new(c_m);

        let mock_state = AppState::new(Box::new(MockDataBase), cache);

        let router = Router::new()
            .route("/info", get(|| async { "Ok" }))
            .layer(middleware::from_fn_with_state(
                mock_state.clone(),
                auth_middleware,
            ))
            .with_state(mock_state);

        let server = TestServer::new(router).unwrap();

        // Act
        let response = server
            .get("/info")
            .add_header("Authorization", "Bearer 123")
            .await;

        // Assertion
        response.assert_status_ok();

        client.del("session:123").unwrap();
    }
}
