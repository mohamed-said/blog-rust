use axum::{http::StatusCode, response::IntoResponse};

use super::client_error::ClientError;

pub type Result<T> = core::result::Result<T, UserError>;

#[derive(Debug)]
pub enum UserError {
    UserNotFound,
    UserNotAdded,
    UserAlreadyExists,
}

impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:12} - {self:?}", "INTO_RES");

        (StatusCode::NOT_FOUND, "ARTICLE_NOT_FOUND").into_response()
    }
}

impl UserError {
    fn _client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::UserNotFound => (StatusCode::NOT_FOUND, ClientError::INVALID_PARAMS),
            Self::UserNotAdded => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
            Self::UserAlreadyExists => (StatusCode::BAD_REQUEST, ClientError::SERVICE_ERROR),
        }
    }
}
