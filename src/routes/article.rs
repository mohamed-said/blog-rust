use axum::{
    extract::{Path, State}, routing::{delete, post}, Json, Router
};

use crate::{ctx::ctx::Ctx, models::article::{AddArticleRequest, Article, ArticleController}};
use crate::error::article_error::Result;

pub fn routes(mc: ArticleController) -> Router {
    Router::new()
        .route("/article", post(add_article).get(get_articles))
        .route("/article/:id", delete(delete_article))
        .with_state(mc)
}

async fn add_article(
    State(mc): State<ArticleController>,
    ctx: Ctx,
    Json(req): Json<AddArticleRequest>
) -> Result<Json<Article>> {
    println!("->> {:12} - {}", "HANDLER", "add_article");
    let article = mc.create_article(ctx, req).await?;

    Ok(Json(article))
}


async fn get_articles (
    State(mc): State<ArticleController>,
    ctx: Ctx,
) -> Result<Json<Vec<Article>>> {
    println!("->> {:12} - {}", "HANDLER", "get_articles");
    let articles = mc.list_articles(ctx).await?;

    Ok(Json(articles))
}

async fn delete_article(
    State(mc): State<ArticleController>,
    ctx: Ctx,
    Path(id): Path<u64>
) -> Result<Json<Article>> {
    println!("->> {:12} - {}", "HANDLER", "delete_article");
    let deleted_article = mc.delete_article(ctx, id).await?;

    Ok(Json(deleted_article))
}
