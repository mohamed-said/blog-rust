use axum::{http::StatusCode, response::IntoResponse};

use super::{Error, client_error::ClientError};

pub type Result<T> = core::result::Result<T, AuthError>;

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum AuthError {
    NoAuthTokenCookie,
    InvalidTokenFormat,
    CtxNotInRequestExtension,
}

impl IntoResponse for AuthError  {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:12} - {self:?}", "INTO_RES");

        // create a placeholder for Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // insert the error into the response
        response.extensions_mut().insert(self);

        response
    }
}

impl Error for AuthError {
    fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        // FIXME: remove unnecessary patterns and make sure everything is exhaustive
        #[allow(unreachable_patterns)]
        match self {
            AuthError::NoAuthTokenCookie | AuthError::InvalidTokenFormat | AuthError::CtxNotInRequestExtension => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }
            _ => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR)
            }
        }
    }
}
