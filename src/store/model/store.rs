use crate::config::config::Config;
use crate::errors::errors::*;
use crate::store::mock_store::MockStore;
use crate::store::model::actix_store::ActixStore;
use crate::store::scylla_session::create_db_session;

use std::sync::Arc;

#[derive(Clone)]
pub struct Store {
    pub mock_store: MockStore,
    pub actix_store: Arc<ActixStore>,
}

impl Store {
    pub async fn new(config: &Config) -> std::result::Result<Store, AppError> {
        let mock_store = MockStore::new().await?;

        let session = Arc::new(create_db_session(&config.scylla_config).await?);

        let actix_store = Arc::new(ActixStore::new(session.clone(),
                                                   &config.scylla_config.keyspace,
                                                   "actix").await?);

        Ok(Store {
            mock_store,
actix_store,
        })
    }
}
