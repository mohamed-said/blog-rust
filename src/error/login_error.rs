use axum::{http::StatusCode, response::IntoResponse};

use super::client_error::ClientError;

pub type Result<T> = core::result::Result<T, LoginError>;

#[derive(Debug)]
pub enum LoginError {
    LoginFail,
}

impl IntoResponse for LoginError {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

impl LoginError {
    fn _client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
        }
    }
}
