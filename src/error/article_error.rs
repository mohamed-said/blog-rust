use axum::{http::StatusCode, response::IntoResponse};

use super::client_error::ClientError;

pub type Result<T> = core::result::Result<T, ArticleError>;

#[derive(Debug)]
pub enum ArticleError {
    ArticleIdNotFound,
    ArticleNotAdded,
}

impl IntoResponse for ArticleError {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:12} - {self:?}", "INTO_RES");

        (StatusCode::NOT_FOUND, "ARTICLE_NOT_FOUND").into_response()
    }
}

impl ArticleError {
    fn _client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::ArticleIdNotFound => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),
            Self::ArticleNotAdded => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}
