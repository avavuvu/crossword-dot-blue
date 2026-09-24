use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;

pub enum AppError {
    NotFound,
    BadRequest,
    Internal(String),
}

pub type AppResult<T = Response> = Result<T, AppError>;

impl From<DbErr> for AppError {
    fn from(e: DbErr) -> Self {
        AppError::Internal(format!("database: {e}"))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Internal(format!("json: {e}"))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND.into_response(),
            AppError::BadRequest => StatusCode::BAD_REQUEST.into_response(),
            AppError::Internal(message) => {
                eprintln!("[error] {message}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
