use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use bson::oid::ObjectId;

use crate::error::article_error::Result;
use crate::models::article::{AddArticleRequest, Article, ArticleController};

pub fn routes(mc: ArticleController) -> Router {
    Router::new()
        .route("/article", post(add_article))
        .route("/list", get(get_articles))
        .with_state(mc)
}

async fn add_article(
    State(mc): State<ArticleController>,
    Json(req): Json<AddArticleRequest>,
) -> Result<Json<ObjectId>> {
    println!("->> {:12} - {}", "HANDLER", "add_article");

    let added_object_id = mc.create_article(req).await?;

    Ok(Json(added_object_id))
}

async fn get_articles(State(mc): State<ArticleController>) -> Result<Json<Vec<Article>>> {
    println!("->> {:12} - {}", "HANDLER", "get_articles");

    let articles = mc.list_articles().await?;

    println!("how many articles?: {}", &articles.len());

    Ok(Json(articles))
}
