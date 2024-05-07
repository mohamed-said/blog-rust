use std::env;

use futures::stream::StreamExt;

use bson::{doc, extjson::de::Error};
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};

use crate::models::article::Article;

pub struct Database {
    articles: Collection<Article>,
}

impl Database {
    pub async fn init() -> Database {
        let client_uri = match env::var("MONGODB_URI") {
            Ok(value) => value.to_string(),
            Err(_) => String::from("mongodb://localhost:27017/?directConnection=true")
        };

        let options = ClientOptions::parse_with_resolver_config(
            &client_uri,
            ResolverConfig::cloudflare()
        )
            .await
            .unwrap();

        let client = Client::with_options(options).unwrap();
        let db = client.database("blog-database");

        let articles: Collection<Article> = db.collection("article");

        Database {
            articles
        }
    }

    // TODO
    // check how to use doc! for insert_one instead of creatign an object
    pub async fn seed_articles(&self) {
        let article1 = Article {
            id: 23453453,
            user_id: 23423423,
            title: String::from("First Article"),
            body: String::from("First Body")
        };

        let article2 = Article {
            id: 34543345,
            user_id: 23423423,
            title: String::from("Second Article"),
            body: String::from("Second Body")
        };

        let article3 = Article {
            id: 1256544,
            user_id: 23423423,
            title: String::from("Third Article"),
            body: String::from("Third Body")
        };

        self.articles.insert_one(
            article1,
            None
        ).await.unwrap();

        self.articles.insert_one(
            article2,
            None
        ).await.unwrap();

        self.articles.insert_one(
            article3,
            None
        ).await.unwrap();
    }

    pub async fn get_all_articles(&self) -> Result<Vec<Article>, Error> {
        let mut results = self
            .articles
            .find(None, None).await.map_err(|_e| panic!("lol!")).unwrap();

        let mut articles: Vec<Article> = Vec::new();

        while let Some(result) = results.next().await {
            match result {
                Ok(doc) => {
                    articles.push(doc);
                }
                Err(e) => {
                    panic!("Error getting article: {}", e);
                }
            }
        }

        Ok(articles)
    }

    pub async fn delete_all_articles(&self) {
        let _ = self.articles.delete_many(doc! {}, None).await;
    }

    pub async fn add_article(&mut self, article: Article) {
        let _ = self.articles.insert_one(article, None).await;
    }

}
