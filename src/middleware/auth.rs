use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::Response;
use axum::{extract::Request, middleware::Next};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::ctx::ctx::Ctx;
use crate::routes::AUTH_TOKEN;
use crate::error::auth_error::{AuthError, Result};

pub async fn mw_ctx_resolver(
    // TODO: use this later
    //_mc: State<ModelController>, // could be the databse connection later
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Result<Response> {
    println!("->> {:12} - {}", "MIDDLEWARE", "mw_ctx_resolver");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Compute Result<Ctx>
    let result_ctx = match auth_token
        .ok_or(AuthError::NoAuthTokenCookie)
        .and_then(parse_token) {
            Ok ((user_id, _exp, _sign)) => {
                // Token component validation
                // e.g. check in the databse if the token and user_id
                // actually correct and not malicious
                Ok(Ctx::new(user_id))
            }
            Err(e) => Err(e),
        };

    // remove the cookie if any unhandled error happens

    if result_ctx.is_err() && !matches!(result_ctx, Err(AuthError::NoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    req: Request,
    next: Next,
) -> Result<Response> {
    println!("->> {:12} - {}", "MIDDLEWARE", "my_require_auth");

    ctx?;

    Ok(next.run(req).await)
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_, user_id, exp, signature) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, &token
    ).ok_or(AuthError::InvalidTokenFormat)?;

    let user_id: u64 = user_id.parse().map_err(|_| AuthError::InvalidTokenFormat)?;

    Ok((user_id, exp.to_string(), signature.to_string()))
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Ctx> {
        println!("->> {:12} - {}", "EXTRACTOR", "Ctx");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(AuthError::CtxNotInRequestExtension)?
            .clone()
    }
}
