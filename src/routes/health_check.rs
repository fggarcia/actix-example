use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref VERSION: &'static str = version!();
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    status: String,
    version: &'static str,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
        version: *VERSION,
    })
}

pub async fn version() -> impl Responder {
    HttpResponse::Ok().json(*VERSION)
}
