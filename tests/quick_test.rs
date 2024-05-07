#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_test() -> Result<()> {
    let http_client = httpc_test::new_client("http://0.0.0.0:3000")?;

    http_client.do_get("/articles").await?.print().await?;

    let req_login = http_client.do_post(
        "/api/login",
        json!({
            "username":"admin",
            "password":"admin",
        })
    );
    req_login.await?.print().await?;

    let req_add_article = http_client.do_post(
        "/api/article",
        json!({
            "title": "Article ONE",
            "body": "Hello Booooodyyyy!"})
    );
    req_add_article.await?.print().await?;

    http_client.do_get("/api/article").await?.print().await?;

    http_client.do_delete("/api/article/423423423").await?.print().await?;

    http_client.do_get("/api/article").await?.print().await?;

    Ok(())
}
