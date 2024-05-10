use axum::{async_trait, extract::FromRequestParts, http::{header, request::Parts, Response, StatusCode}, routing::post, Json, Router};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn routes() -> Router {
    Router::new()
        .route("/get-token", post(get_token))
}

#[derive(Deserialize, Serialize)]
struct Claims {
    email: String,
    exp: i64,
}

#[derive(Deserialize, Serialize)]
pub struct UserToken {
    email: String,
}

// Custom Extractor
pub struct Auth(pub UserToken);

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Auth {
    type Rejection = Response<String>;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let access_token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.split(" ").nth(1));

        match access_token {
            Some(token) => {
                let user = decode_jwt(token);
                match user {
                    Ok(user) => {
                        Ok(Auth(user))
                    }
                    Err(e) => {
                        Err(
                            Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .header(header::CONTENT_TYPE, "application/json")
                            .body(json!({
                                "success": false,
                                "data": {
                                    "message": e.to_string()
                                }
                            }).to_string())
                            .unwrap_or_default())
                    }
                }
            }
            None => {
                Err(
                    Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(json!({
                        "success": false,
                        "data": {
                            "message": "No token provided"
                        }
                    }).to_string())
                    .unwrap_or_default())
            }
        }

    }
}

pub async fn get_token(Json(user): Json<UserToken>) -> Response<String> {
    let token = get_jwt(user);

    match token {
        Ok(token) => {
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(json!({
                    "success": true,
                    "data": {
                        "token": token
                    }
                }).to_string())
            .unwrap_or_default()
        }
        Err(e) => {
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(header::CONTENT_TYPE, "application/json")
                .body(json!({
                    "success": false,
                    "data": {
                        "message": e.to_string()
                    }
                }).to_string())
            .unwrap_or_default()
        }
    }
}



pub fn get_jwt(user: UserToken) -> Result<String, String> {
    let claims = Claims {
        email: user.email,
        exp: (Utc::now() + Duration::minutes(1)).timestamp()
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("mykey".as_bytes())
    ).map_err(|e| e.to_string());

    token
}

fn decode_jwt(token: &str) -> Result<UserToken, String> {
    let token_data = decode::<UserToken>(
        token,
        &DecodingKey::from_secret("mykey".as_bytes()),
        &Validation::default()
    );

    match token_data {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => Err(e.to_string())
    }
}
