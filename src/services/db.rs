use std::env;

use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};

pub struct Database;

impl Database {
    pub async fn init() -> mongodb::Database {
        let client_uri = match env::var("MONGODB_URI") {
            Ok(value) => value.to_string(),
            Err(_) => String::from("mongodb://localhost:27017/?directConnection=true"),
        };

        let options =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
                .await
                .unwrap();

        let client = Client::with_options(options).unwrap();
        let db = client.database("blog-database");

        db
    }
}
