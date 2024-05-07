use axum::http::StatusCode;

use self::client_error::ClientError;

pub mod client_error;
pub mod login_error;
pub mod article_error;
pub mod auth_error;

pub trait Error {
    fn client_status_and_error(&self) -> (StatusCode, ClientError);
}
