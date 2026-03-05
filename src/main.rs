use crate::{
    auth::create_auth_router,
    configuration::Configuration,
    dal::{
        cache_repo::CacheRepo,
        persistent_repo::{PersistentRepo, RealPersistentRepo},
    },
    jobs::create_periodic_movie_fetch_job,
};
use anyhow::Result;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};

mod auth;
mod configuration;
mod connectors;
mod dal;
mod jobs;
mod net;
mod utils;

#[derive(Clone)]
pub struct AppState {
    storage: Arc<dyn PersistentRepo>,
    cache: CacheRepo,
}

impl AppState {
    pub fn new(storage: Box<dyn PersistentRepo>, cache: CacheRepo) -> Self {
        AppState {
            storage: Arc::from(storage),
            cache,
        }
    }

    pub fn get_storage(&self) -> &dyn PersistentRepo {
        self.storage.as_ref()
    }

    pub fn get_cache(&self) -> &CacheRepo {
        &self.cache
    }

    pub fn get_cache_mut(&mut self) -> &mut CacheRepo {
        &mut self.cache
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    // Parse env file
    dotenvy::from_filename("dev.env")?;

    let config = Configuration::new(
        env::var("HOST")?,
        env::var("PORT")?,
        env::var("DATABASE_URL")?,
        env::var("REDIS_URL")?,
    );

    let storage = configure_db(&config).await?;

    let state = AppState::new(storage, confugire_cache(&config).await?);

    let address = config.get_address();

    let router = Router::new()
        .nest("/auth", create_auth_router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(address).await?;

    let job = create_periodic_movie_fetch_job().await?;
    job.start().await?;

    axum::serve(listener, router).await?;

    Ok(())
}

async fn configure_db(conf: &Configuration) -> Result<Box<dyn PersistentRepo>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(conf.get_connection_string())
        .await?;

    Ok(Box::new(RealPersistentRepo::new(pool)))
}

async fn confugire_cache(conf: &Configuration) -> Result<CacheRepo> {
    let client = redis::Client::open(conf.get_redis_url())?;
    let conn = client.get_connection_manager().await?;

    Ok(CacheRepo::new(conn))
}
