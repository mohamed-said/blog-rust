use axum::{http::StatusCode, response::IntoResponse};

use super::{
    Error,
    client_error::ClientError
};

pub type Result<T> = core::result::Result<T, ArticleError>;

#[derive(Debug)]
pub enum ArticleError {
    ArticleIdNotFound,
}

impl IntoResponse for ArticleError  {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:12} - {self:?}", "INTO_RES");

        (StatusCode::NOT_FOUND, "ARTICLE_NOT_FOUND").into_response()
    }
}

impl Error for ArticleError {
    fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::ArticleIdNotFound => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
        }
    }
}
