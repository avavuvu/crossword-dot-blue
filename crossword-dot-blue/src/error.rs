use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use boutique::{htmx::fragments, validator::ValidationErrors};
use sea_orm::DbErr;

pub enum AppError {
    NotFound,
    BadRequest,
    Internal(String),
    Validation(ValidationErrors),
    Fields(Vec<(&'static str, String)>),
    Message(String),
}

pub type AppResult<T = Response> = Result<T, AppError>;

impl AppError {
    pub fn field(field: &'static str, message: impl Into<String>) -> Self {
        AppError::Fields(vec![(field, message.into())])
    }

    pub fn message(message: impl Into<String>) -> Self {
        AppError::Message(message.into())
    }

    pub fn internal(context: &str, error: impl std::fmt::Display) -> Self {
        AppError::Internal(format!("{context}: {error}"))
    }
}

impl From<DbErr> for AppError {
    fn from(e: DbErr) -> Self {
        AppError::internal("database", e)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::internal("json", e)
    }
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        AppError::Validation(errors)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND.into_response(),
            AppError::BadRequest => StatusCode::BAD_REQUEST.into_response(),
            AppError::Internal(message) => {
                eprintln!("[error] {message}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    fragments::error("Something went wrong. Please try again."),
                )
                    .into_response()
            }
            AppError::Validation(errors) => {
                (StatusCode::UNPROCESSABLE_ENTITY, fragments::from_errors(errors)).into_response()
            }
            AppError::Fields(fields) => {
                let fields: Vec<(&str, Option<&str>)> =
                    fields.iter().map(|(field, message)| (*field, Some(message.as_str()))).collect();
                (StatusCode::UNPROCESSABLE_ENTITY, fragments::field_errors(&fields)).into_response()
            }
            AppError::Message(message) => {
                (StatusCode::UNPROCESSABLE_ENTITY, fragments::error(&message)).into_response()
            }
        }
    }
}
