use worker::*;

pub enum AppError {
    NotFound,
    Internal(String),
}

impl From<AppError> for Response {
    fn from(err: AppError) -> Self {
        match err {
            AppError::NotFound => Response::error("Not Found", 404).unwrap(),
            AppError::Internal(s) => Response::error(s, 500).unwrap(),
        }
    }
}
