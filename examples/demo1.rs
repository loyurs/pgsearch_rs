//测试pipeline

use anyhow::Error;
use futures::future;
use mysearch_tools::psql::Pgconfig;
use std::future::Future;
use tokio_postgres::{Client, Config, Error, NoTls, Statement};

// async fn pipelined_prepare(client: &Client) -> Result<(Statement, Statement), Error> {
//     future::try_join(
//         client.prepare("SELECT * FROM foo"),
//         client.prepare("INSERT INTO bar (id, name) VALUES ($1, $2)"),
//     )
//     .await
// }

#[tokio::main]
async fn main() {
    let client = new_client().await.unwrap();
    pipelined_prepare(&client).await;
}
pub async fn new_client() -> Result<Client, Error> {
    let toml_content = tokio::fs::read_to_string(std::path::Path::new("info.toml")).await?;
    //结构需要和info.toml中的[var]一致
    let info_config: Pgconfig = toml::from_str(&toml_content).unwrap();
    let mut config = Config::new();
    config
        .host(info_config.postgres.get("host").unwrap())
        .user(info_config.postgres.get("user").unwrap())
        .dbname(info_config.postgres.get("database").unwrap())
        .password(info_config.postgres.get("password").unwrap())
        .port(
            (info_config.postgres.get("port").unwrap())
                .parse::<u16>()
                .unwrap(),
        );
    let (client, connection) = config.connect(NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}

