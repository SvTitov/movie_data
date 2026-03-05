use crate::net::{CreateUserDto, LoginDto, LoginResponse};
use crate::AppState;
use axum::http::StatusCode;
use axum::{extract::State, routing::post, Json, Router};

mod auth_middleware;

pub fn create_auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/create_user", post(create_user))
}

async fn login(
    State(mut app): State<AppState>,
    Json(payload): Json<LoginDto>,
) -> Result<Json<LoginResponse>, StatusCode> {
    if payload.login.is_empty() | payload.password.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let valid_user = app
        .get_storage()
        .user_valid(&payload.login, &payload.password)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    if valid_user {
        let token = app
            .get_cache_mut()
            .create_session(&payload.login)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        return Ok(Json(LoginResponse::new(token)));
    }

    return Err(StatusCode::BAD_REQUEST);
}

async fn create_user(State(app): State<AppState>, Json(payload): Json<CreateUserDto>) {
    match app
        .get_storage()
        .create_user(&payload.login, &payload.password, chrono::Utc::now())
        .await
    {
        Ok(_) => tracing::info!("User was succesfully created!"),
        Err(err) => tracing::error!("{err}"),
    }
}

mod test {
    use std::{sync::Arc, time::Duration};

    use axum::{extract::State, routing::post, Json, Router};
    use axum_test::TestServer;
    use chrono::Utc;
    use sqlx::PgPool;
    use tokio::time::sleep;

    use crate::{
        auth::create_user,
        dal::persistent_repo::{PersistentRepo, RealPersistentRepo},
        net::CreateUserDto,
    };

    #[tokio::test]
    async fn should_create_user_with_valid_data() {
        // Arrange
        let pool = PgPool::connect("postgresql://test:test@localhost:5432/test_db")
            .await
            .expect("Cannot connect to the corresponding db!");

        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        let repo = Arc::new(RealPersistentRepo::new(pool.clone()));
        let router = Router::new().route(
            "/create_user",
            post({
                let s_ref = repo.clone();
                async move |Json(payload): Json<CreateUserDto>| {
                    _ = s_ref
                        .create_user(&payload.login, &payload.password, Utc::now())
                        .await;
                }
            }),
        );

        let server = TestServer::new(router).unwrap();

        let dto = CreateUserDto {
            login: "test".to_string(),
            password: "test123".to_string(),
        };

        // Act
        let response = server.post("/create_user").json(&dto).await;

        // Assert
        response.assert_status_ok();

        sqlx::query("DELETE FROM users WHERE login = $1")
            .bind("test")
            .execute(&pool)
            .await
            .unwrap();
    }
}

