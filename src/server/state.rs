use crate::store::model::store::Store;

#[derive(Clone)]
pub struct AppState {
    pub store: Store,
}
