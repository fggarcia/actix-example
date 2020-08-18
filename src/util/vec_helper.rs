use crate::errors::errors::{AppError, AppErrorType};

pub fn empty_resource<T>(resources: Vec<T>, msg: String) -> Result<Vec<T>, AppError> {
    if resources.is_empty() {
        Err(AppError {
            message: Some(msg),
            cause: None,
            error_type: AppErrorType::NotFoundError,
        })
    } else {
        Ok(resources)
    }
}
