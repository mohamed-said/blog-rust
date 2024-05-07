use std::{collections::HashMap, fmt::Display, sync::Arc};

use crate::{ctx::ctx::Ctx, error::article_error::{ArticleError, Result}};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Article {
    pub id: u64,
    pub user_id: u64, // article creator id
    pub title: String,
    pub body: String,
}

impl Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, " --------- Article ---------\n\ttitle: {}\n\tbody: {}\n", self.title, &self.body)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddArticleRequest {
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddArticleResponse {
    pub status: u16,
    pub message: String,
}

#[derive(Clone)]
pub struct ArticleController {
   articles: Arc<Mutex<HashMap<u64, Article>>>
}

impl ArticleController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            articles: Arc::default()
        })
    }

    pub async fn create_article(&self, ctx: Ctx, req: AddArticleRequest) -> Result<Article> {
        let mut store = self.articles.lock().await;

        let id = 423423423 as u64;

        let article = Article {
            id,
            user_id: ctx.user_id(),
            title: req.title,
            body: req.body,
        };

        store.insert(id, article.clone());

        Ok(article)
    }

    pub async fn list_articles(&self, ctx: Ctx) -> Result<Vec<Article>> {
        let store = self.articles.lock().await;

        let mut res: Vec<Article> = vec![];

        for (&id, article) in store.iter() {
            res.push(Article {
                id,
                user_id: ctx.user_id(),
                title: article.title.clone(),
                body: article.body.clone(),
            });
        }

        Ok(res)
    }

    pub async fn delete_article(&self, _ctx: Ctx, id: u64) -> Result<Article> {
        let mut store = self.articles.lock().await;

        if let Some(res) = store.remove_entry(&id) {
            Ok(res.1)
        } else {
            Err(ArticleError::ArticleIdNotFound)
        }
    }
}
