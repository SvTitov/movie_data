use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    connectors::omdb::OmdbConnector,
    dal::{cache_repo::CacheRepo, persistent_repo::PersistentRepo},
};

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

    pub fn get_storage(&self) -> Arc<dyn PersistentRepo> {
        self.storage.clone()
    }

    pub fn get_cache(&self) -> Arc<Mutex<CacheRepo>> {
        self.cache.clone()
    }

    pub fn get_omdb_connector(&self) -> Arc<OmdbConnector> {
        self.omdb_connector.clone()
    }
}
