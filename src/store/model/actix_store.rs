use crate::store::scylla_session::CurrentSession;
use crate::errors::errors::{AppError, AppErrorType};
use crate::api::store_model::{StoreModelQuery, StoreModel};
use crate::store::scylla_session::create_keyspace;

use cdrs::consistency::Consistency;
use cdrs::{query::*, query_values, types::prelude::*};
use fomat_macros::fomat;
use std::sync::Arc;
use tracing::{error, info};

#[derive(Clone)]
pub struct ActixStore {
    pub current_session: Arc<CurrentSession>,
    insert_query: String,
    select_query: String,
}

impl ActixStore {
    pub async fn new(
        session: Arc<CurrentSession>,
        keyspace: &str,
        table_name: &str,
    ) -> std::result::Result<ActixStore, AppError> {
        let _create_keyspace =
            create_keyspace(session.clone(), keyspace, 1).await?;
        let _create_table = create_table(session.clone(), keyspace, table_name).await?;

        let insert_query = create_insert_query(keyspace, table_name);
        let select_query = create_select_query(keyspace, table_name);

        Ok(ActixStore {
            current_session: session,
            insert_query,
            select_query,
        })
    }
}

async fn create_table(
    session: Arc<CurrentSession>,
    keyspace: &str,
    table_name: &str,
) -> std::result::Result<(), AppError> {
    let create_table = fomat!(
        ("CREATE TABLE IF NOT EXISTS ")(keyspace)"."(table_name) " (\n"
        "\t name text, \n"
        "\t reference text, \n"
        "\t num int, \n"
        "\t num2 int, \n"
        "\t PRIMARY KEY(name, reference) \n"
        ");"
    );
    info!("creating table: {}", create_table);
    session.query(create_table).await?;
    Ok(())
}

fn create_insert_query(keyspace: &str, table_name: &str) -> String {
    fomat!("INSERT INTO "(keyspace)"."(table_name)" (name, reference, num, num2) "
    "VALUES (?, ?, ?, ?);")
}

fn create_select_query(keyspace: &str, table_name: &str) -> String {
    fomat!("SELECT * FROM "(keyspace)"."(table_name)" "
    "WHERE name = ?;")
}

fn select_query_values(store: &ActixStore, query: StoreModelQuery) -> (QueryParams, String) {
    let mut query_params = QueryParamsBuilder::new();
    query_params = query_params.consistency(Consistency::LocalOne);

    query_params = query_params.values(query_values!(
                query.name.as_str()
            ));

    (query_params.finalize(), store.select_query.clone())
}

pub async fn select(
    store: &ActixStore,
    query: StoreModelQuery,
) -> std::result::Result<Vec<StoreModel>, AppError> {
    let (query_values, query_str) = select_query_values(&store, query);

    let rows_result =
        store.current_session.query_with_params(query_str, query_values)
        .await?
        .get_body()?
        .into_rows()
        .ok_or(AppError {
            cause: None,
            message: Some("error retrieving rows".to_string()),
            error_type: AppErrorType::DbError,
        })?;

    let mut rows: Vec<StoreModel> = Vec::with_capacity(rows_result.len());
    for row in rows_result {
        let elem: StoreModel = StoreModel::try_from_row(row)?;
        rows.push(elem);
    }
    Ok(rows)
}

pub async fn insert(
    store: &ActixStore,
    store_model: &StoreModel,
) -> std::result::Result<bool, AppError> {

    let result = store.current_session.query_with_values(store.insert_query.clone(), insert_query_values_for(store_model)).await
        .map(|_| {
            true
        })
        .map_err(|err| {
            error!("error saving {:?}", err);
            err
        })?;

    Ok(result)
}

fn insert_query_values_for(store_model: &StoreModel) -> QueryValues {
    query_values!(
        store_model.name.as_str(),
        store_model.reference.as_str(),
        store_model.num,
        store_model.num2
    )
}
