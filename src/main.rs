use crate::{
    app_state::AppState,
    auth::create_auth_router,
    configuration::Configuration,
    connectors::omdb::OmdbConnector,
    dal::{
        cache_repo::CacheRepo,
        persistent_repo::{PersistentRepo, RealPersistentRepo},
    },
    jobs::OmdbPeriodicFetcher,
};
use anyhow::Result;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod app_state;
mod auth;
mod configuration;
mod connectors;
mod dal;
mod jobs;
mod net;
mod utils;

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
    let storage = state.get_storage();

    let address = config.get_address();

    let router = Router::new()
        .nest("/auth", create_auth_router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(address).await?;

    let opf = OmdbPeriodicFetcher::new(storage);
    let job = opf.start_fetch(omdb).await?;
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
