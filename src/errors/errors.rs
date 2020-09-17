use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use cdrs::error::Error as CDRSError;
use serde::Serialize;
use std::fmt;
use async_std::future::TimeoutError;
use tracing::error;

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
    TimeOutError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(message),
                ..
            } => message.clone(),
            AppError {
                message: None,
                error_type: AppErrorType::NotFoundError,
                ..
            } => "The request resource was not found".to_string(),
            _ => {
                error!("something {:?}", std::thread::current().name());
                "An unexpected error has occurred".to_string()
            },
        }
    }
}

impl From<CDRSError> for AppError {
    fn from(error: CDRSError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}

impl From<TimeoutError> for AppError {
    fn from(error: TimeoutError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::TimeOutError,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::TimeOutError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}
