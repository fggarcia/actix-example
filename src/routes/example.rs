use crate::api::domain::Domain;
use crate::errors::errors::AppError;
use crate::server::state::AppState;
use crate::service::example_service;
use crate::util::log::log_error;
use crate::util::log::log_with_context;
use crate::util::vec_helper::empty_resource;

use actix_http::http::HeaderValue;
use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
use std::sync::Arc;
use tracing::{info, info_span};
use tracing_futures::Instrument;
use crate::api::query::DomainQuery;

const POST_SEARCH: &str = "/POST/actix-example/example";
const GET: &str = "/GET/actix-example/example";

pub async fn post(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
    json: web::Json<Vec<Domain>>,
) -> Result<impl Responder, AppError> {
    let user_context = log_with_context(&req);
    let searches: Vec<Domain> = json.0;

    let response_f = async move {
        let _ = example_service::store(&state.store, searches).await;
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
    query: web::Query<DomainQuery>,
) -> Result<impl Responder, AppError> {
    let user_context = log_with_context(&req);

    let domain_query: DomainQuery = query.0.into();

    let response_f = async move {
        example_service::query(&state.store, &domain_query)
            .await
            .map(|elems| empty_resource(elems, "No routes found".to_string()))
            .map(|r| r.unwrap())
            .map_err(log_error())
    };

    let response = response_f
        .instrument(info_span!(
            GET,
            %user_context
        ))
        .await?;

    info!("before return from: {:?} {:?}", response, std::thread::current().name());

    Ok(HttpResponse::Ok()
        .json(response)
        .with_header("X-Service", HeaderValue::from_static("GET/actix-example")))
}
