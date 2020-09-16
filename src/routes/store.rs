use crate::api::store_model::{StoreModel, StoreModelQuery};
use crate::errors::errors::AppError;
use crate::server::state::AppState;
use crate::service::store_service;
use crate::util::log::log_error;
use crate::util::log::log_with_context;
use crate::util::vec_helper::empty_resource;

use actix_http::http::HeaderValue;
use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
use std::sync::Arc;
use tracing::info_span;
use tracing_futures::Instrument;

const POST_SEARCH: &str = "/POST/actix-example/store";
const GET: &str = "/GET/actix-example/store";

pub async fn post(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
    json: web::Json<StoreModel>,
) -> Result<impl Responder, AppError> {
    let user_context = log_with_context(&req);
    let model: StoreModel = json.0;

    let response_f = async move {
        let _ = store_service::store(&state.store, model).await;
    };

    actix_rt::spawn(response_f.instrument(info_span!(
        POST_SEARCH,
        %user_context
    )));

    Ok(HttpResponse::Ok()
        .json("OK")
        .with_header("X-Service", HeaderValue::from_static("POST/actix-example")))
}

pub async fn query(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
    query: web::Query<StoreModelQuery>,
) -> Result<impl Responder, AppError> {
    let user_context = log_with_context(&req);

    let store_query: StoreModelQuery = query.0.into();

    let response_f = async move {
        store_service::query(&state.store, store_query)
            .await
            .map(|elems| empty_resource(elems, "No routes found".to_string()))?
            .map(|elems| {
                HttpResponse::Ok()
                    .json(elems)
                    .with_header("X-Service", HeaderValue::from_static("GET/actix-example"))
            })
            .map_err(log_error())
    };

    response_f
        .instrument(info_span!(
            GET,
            %user_context
        ))
        .await
}
