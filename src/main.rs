pub mod ctx;
pub mod services;
pub mod models;
pub mod routes;
pub mod error;
pub mod middleware;

use axum::{middleware::from_fn, response::Response, Router};
use futures::executor::block_on;
use middleware::auth;
use models::article::ArticleController;
use routes::{article, login};
use tokio;
use tower_cookies::CookieManagerLayer;
use error::Error;

#[tokio::main]
async fn main() -> error::article_error::Result<()> {
    let repo = services::db::Database::init().await;
block_on(repo.seed_articles());

    let article_controller = ArticleController::new().await?;

    let routes_apis = routes::article::routes(article_controller.clone())
        .route_layer(from_fn(auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(login::routes())
        .merge(article::routes(article_controller.clone()))
        .nest("/api", routes_apis)
        .layer(axum::middleware::map_response(main_response_mapper))
        .layer(axum::middleware::from_fn(auth::mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes::static_routes::routes_static());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}

// TODO: complete this with a proper logging implementation
async fn main_response_mapper(response: Response) -> Response {
    println!("->> {}: main_response_mapper", "RES_MAPPER");

    let service_error = response.extensions().get::<impl impl Error>();

    println!();

    response
}
