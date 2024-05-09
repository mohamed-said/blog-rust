pub mod ctx;
pub mod error;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;

use axum::{response::Response, routing::get, Router};
use models::{article::ArticleController, user::UserController};
use routes::article;
use tokio;

#[tokio::main]
async fn main() -> error::article_error::Result<()> {
    let db = services::db::Database::init().await;

    let article_controller = ArticleController::new(db.clone());
    let user_controller = UserController::new(db.clone());

    let article_apis = article::routes(article_controller);
    let user_apis = routes::user::routes(user_controller);

    let routes_all = Router::new()
        //.merge(login::routes())
        .merge(Router::new().route("/", get(root)))
        .nest("/api", article_apis)
        .nest("/api", user_apis)
        .layer(axum::middleware::map_response(main_response_mapper));
    //.layer(axum::middleware::from_fn(auth::mw_ctx_resolver))
    //.layer(CookieManagerLayer::new())

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}

// TODO: complete this with a proper logging implementation
async fn main_response_mapper(response: Response) -> Response {
    println!("->> {}: main_response_mapper", "RES_MAPPER");
    println!();

    response
}

async fn root() -> &'static str {
    "Hello world!"
}
