use crate::config::config::Config;
use crate::errors::errors::*;
use crate::store::mock_store::MockStore;

#[derive(Clone)]
pub struct Store {
    pub mock_store: MockStore,
}

impl Store {
    pub async fn new(_config: &Config) -> std::result::Result<Store, AppError> {
        let mock_store = MockStore::new().await?;

        Ok(Store {
            mock_store,
        })
    }
}
