use crate::errors::errors::AppError;
use crate::util::headers_constants::*;

use actix_web::http::HeaderMap;
use actix_web::HttpRequest;
use failure::_core::fmt::Formatter;
use serde::Serialize;
use tracing::error;

#[derive(Debug, Serialize)]
pub struct UserContext {
    pub x_uow: String,
    pub x_request: String,
    pub hostname: String,
}

impl UserContext {
    fn new(x_uow: String, x_request: String, hostname: String) -> UserContext {
        UserContext {
            x_uow,
            x_request,
            hostname,
        }
    }
}

impl core::fmt::Display for UserContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            " host: {} X-UOW: {} X-Request: {}",
            self.hostname.as_str(),
            self.x_uow.as_str(),
            self.x_request.as_str(),
        )
    }
}

pub const ACTIX_LOG_FORMAT: &str = r#"%a -> "%{Host}i" : "%r" %s %b X-UOW: "%{X-UOW}i" X-Request:"%{X-Request}i" "%{Referer}i" "%{User-Agent}i" %D ms"#;

fn get_headers_for(headers: &HeaderMap, header_name: &str, default_value: &str) -> String {
    let header_option = headers.get(header_name);
    let header_value = header_option.map_or(default_value, |header| header.to_str().unwrap());
    String::from(header_value)
}

pub fn log_with_context(request: &HttpRequest) -> UserContext {
    let headers = request.headers();
    let x_uow_value = get_headers_for(headers, X_UOW, "");
    let x_request_value = get_headers_for(headers, X_REQUEST, "");
    UserContext::new(
        x_uow_value,
        x_request_value,
        crate::util::hostname::HOST_NAME.to_string(),
    )
}

pub fn log_error() -> impl Fn(AppError) -> AppError {
    move |err| {
        //let log = log.new(o!(
        //    "cause" => err.cause.clone()
        //));
        error!("{}", err.message());
        err
    }
}
