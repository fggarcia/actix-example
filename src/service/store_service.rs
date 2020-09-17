use crate::api::store_model::{StoreModelQuery, StoreModel};
use crate::errors::errors::AppError;
use crate::store::model::actix_store;
use crate::store::model::store::Store;

use tracing::{debug, error};
use crate::store::model::actix_store::ActixStore;
use std::sync::Arc;

pub async fn query(store: Arc<ActixStore>, query: StoreModelQuery) -> Result<Vec<StoreModel>, AppError> {
    debug!("debugging : {:?}", query);

    actix_store::select(store, query).await
        .map_err(|err| {
            error!("error {:?}", err);
            err
        })
}

pub async fn store(store: &Store, elem: StoreModel) -> Result<(), AppError> {
    debug!("receiving {:?}", elem);

    let _result = actix_store::insert(&store.actix_store, &elem).await;

    Ok(())
}
