pub mod error;
pub mod models;
pub mod routes;
pub mod services;

use axum::{response::Response, routing::get, Router};
use models::{article::ArticleController, user::UserController};
use routes::article;


#[tokio::main]
async fn main() -> error::article_error::Result<()> {
    let db = services::db::Database::init().await;

    let article_controller = ArticleController::new(db.clone());
    let user_controller = UserController::new(db.clone());

    let article_apis = article::routes(article_controller);
    let user_apis = routes::user::routes(user_controller);

    let routes_all = Router::new()
        .merge(Router::new().route("/", get(root)))
        .nest("/api", article_apis)
        .nest("/api", user_apis)
        .layer(axum::middleware::map_response(main_response_mapper));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}

// TODO: complete this with a proper logging implementation
async fn main_response_mapper(response: Response) -> Response {
    println!("->> RES_MAPPER: main_response_mapper");
    println!();

    response
}

async fn root() -> &'static str {
    "Hello world!"
}
