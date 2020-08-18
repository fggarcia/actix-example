use crate::errors::errors::AppError;
use crate::api::query::DomainQuery;

#[derive(Clone)]
pub struct MockStore;

impl MockStore {
    pub async fn new(
    ) -> std::result::Result<MockStore, AppError> {
        Ok(MockStore{})
    }
}

async fn select_internal() -> Result<Vec<usize>, AppError> {
    Ok(vec![1,2,3])
}

pub async fn select(
    _store: &MockStore,
    _query: &DomainQuery,
) -> std::result::Result<Vec<usize>, AppError> {
    select_internal().await
}

pub async fn insert(
    _store: &MockStore,
    _items: Vec<usize>,
) -> std::result::Result<bool, AppError> {
    Ok(true)
}