use crate::{
    auth::create_auth_router,
    configuration::Configuration,
    connectors::omdb::OmdbConnector,
    dal::{
        cache_repo::CacheRepo,
        persistent_repo::{PersistentRepo, RealPersistentRepo},
    },
    jobs::create_periodic_movie_fetch_job,
};
use anyhow::Result;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::{env, os::linux::raw::stat, sync::Arc};
use tokio::sync::Mutex;

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
    omdb_connector: Arc<OmdbConnector>,
    cache: Arc<Mutex<CacheRepo>>,
}

impl AppState {
    pub fn new(
        storage: Box<dyn PersistentRepo>,
        cache: CacheRepo,
        omdb_connector: OmdbConnector,
    ) -> Self {
        AppState {
            storage: Arc::from(storage),
            cache: Arc::new(Mutex::new(cache)),
            omdb_connector: Arc::new(omdb_connector),
        }
    }

    pub fn get_storage(&self) -> &dyn PersistentRepo {
        self.storage.as_ref()
    }

    pub fn get_cache(&self) -> Arc<Mutex<CacheRepo>> {
        self.cache.clone()
    }

    pub fn get_omdb_connector(&self) -> Arc<OmdbConnector> {
        self.omdb_connector.clone()
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
        env::var("OMDB_API_KEY")?,
    );

    let state = AppState::new(
        configure_db(&config).await?,
        confugire_cache(&config).await?,
        configure_omdb_connector(&config)?,
    );

    let omdb = state.get_omdb_connector();

    let address = config.get_address();

    let router = Router::new()
        .nest("/auth", create_auth_router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(address).await?;

    let job = create_periodic_movie_fetch_job(omdb).await?;
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

fn configure_omdb_connector(conf: &Configuration) -> Result<OmdbConnector> {
    Ok(OmdbConnector::new(conf.get_omdb_api_key()))
}
