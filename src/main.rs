//操作表名为search_tables;

use anyhow::Error;
use axum::extract::Extension;
use axum::{routing::post, Router};
use pgsearch_rs::dbconn;
use pgsearch_rs::psql::{self, create_table, distinct_pgsql, Pgconfig};
use pgsearch_rs::webserveapi::{msg_insert_into_db, query_from_postgres};
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //连接数据库
    tracing_subscriber::fmt::init();
    info!("CREATE POOL SUCCESSFUL");
    let pool = dbconn::new_postgres_pool("/root/workspace/Pgsearch_rs/info.toml").await?;
    //创建一个共享状态的扩展
    {
        let client = pool.get().await.unwrap();
        create_table(&client, "search_tables").await.unwrap();
        info!("CREATE TABLE {} SUCCESSFUL", "search_tables");
    }

    let pool_arc = Arc::new(pool);
    let pool1 = pool_arc.clone();
    let pool2 = Arc::clone(&pool_arc);
    let pool3 = Arc::clone(&pool_arc);

    let jebd = jieba_rs::Jieba::new();
    let jeb = Arc::new(jebd);
    let jeb1 = Arc::clone(&jeb);
    let jeb2 = Arc::clone(&jeb);

    //axum通过extension共享状态，可以在扩展中访问共享状态
    let app = Router::new()
        .route("/search", post(query_from_postgres))
        .layer(Extension(pool2))
        .layer(Extension(jeb1))
        .route("/insert", post(msg_insert_into_db))
        .layer(Extension(pool3))
        .layer(Extension(jeb2));

    tokio::spawn(async move {
        loop {
            info!("START DISTINCT FUNCTION");
            let client = pool1.get().await.unwrap();
            distinct_pgsql(&client, "search_tables", "descriptions")
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_secs(100)).await;
        }
    });

    info!("start web server listen on port 0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
