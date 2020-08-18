use crate::errors::errors::AppError;
use crate::store::mock_store;
use crate::store::model::store::Store;

use tracing::{debug, error};
use crate::api::query::DomainQuery;
use crate::api::domain::Domain;

pub async fn query(store: &Store, query: &DomainQuery) -> Result<Vec<usize>, AppError> {
    let selected_store = &store.mock_store;

    mock_store::select(selected_store, query).await
}

pub async fn store(store: &Store, elems: Vec<Domain>) -> Result<(), AppError> {
    debug!("receiving {:?}", elems);

    for item in elems.iter() {
        if !item.vec.is_empty() {
            let _ = mock_store::insert(&store.mock_store, item.vec.clone())
                .await
                .map(|r| debug!("saved vec {:?}", r))
                .map_err(|err| {
                    error!("error saving vec {:?}", err.error_type);
                    err
                });
        } else {
            debug!("avoid inserting vec.....empty");
        }
    }

    Ok(())
}
