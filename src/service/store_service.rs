use crate::api::store_model::{StoreModelQuery, StoreModel};
use crate::errors::errors::AppError;
use crate::store::model::actix_store;
use crate::store::model::store::Store;

use tracing::debug;

pub async fn query(store: &Store, query: StoreModelQuery) -> Result<Vec<StoreModel>, AppError> {
    debug!("debugging : {:?}", query);

    actix_store::select(&store.actix_store, query).await
}

pub async fn store(store: &Store, elem: StoreModel) -> Result<(), AppError> {
    debug!("receiving {:?}", elem);

    let _result = actix_store::insert(&store.actix_store, &elem).await;

    Ok(())
}
