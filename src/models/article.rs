use std::fmt::Display;

use crate::error::article_error::{ArticleError, Result};
use bson::{doc, oid::ObjectId, Document};
use futures::StreamExt;
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Article {
    pub _id: String,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArticleBson {
    pub _id: ObjectId,
    pub title: String,
    pub body: String,
}

impl Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " --------- Article ---------\n\ttitle: {}\n\tbody: {}\n",
            self.title, &self.body
        )
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
    db_instance: Database,
}

impl ArticleController {
    pub fn new(db_instance: Database) -> ArticleController {
        ArticleController { db_instance }
    }

    pub async fn create_article(&self, req: AddArticleRequest) -> Result<ObjectId> {
        let collection = self.db_instance.collection::<Document>("articles");

        let result = collection
            .insert_one(
                doc! {
                    "title": req.title,
                    "body": req.body,
                },
                None,
            )
            .await;

        let inserted_id_bson = match result {
            Ok(r) => r.inserted_id,
            Err(e) => {
                println!("Error: Insert one item failed: {:?}", e);
                return Err(ArticleError::ArticleNotAdded);
            }
        };

        match inserted_id_bson.as_object_id() {
            Some(r) => Ok(r),
            None => {
                println!("Error: Parsing objectId failed");
                Err(ArticleError::ArticleNotAdded)
            }
        }
    }

    pub async fn list_articles(&self) -> Result<Vec<Article>> {
        let mut res: Vec<Article> = vec![];

        let collection = self.db_instance.collection::<Document>("articles");

        let mut query = collection
            .find(None, None)
            .await
            .map_err(|_e| panic!("Error while listiing all articles"))
            .unwrap();

        while let Some(result) = query.next().await {
            match result {
                // FIXME there has to be a better and more idomatic way
                // to deserialize the document into the corresponding type
                Ok(doc) => {
                    let id = doc.get_object_id("_id").unwrap().to_string();
                    let title = doc.get_str("title").unwrap().to_string();
                    let body = doc.get_str("body").unwrap().to_string();
                    let article = Article {
                        _id: id,
                        title,
                        body,
                    };
                    res.push(article);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }

        Ok(res)
    }
}
