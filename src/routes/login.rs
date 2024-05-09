use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{
    error::login_error::{LoginError, Result},
    routes::AUTH_TOKEN,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO impl real db
    if payload.username != "admin" || payload.password != "admin" {
        return Err(LoginError::LoginFail);
    }

    // FIXME: Implement real auth-token generation/signature
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}
