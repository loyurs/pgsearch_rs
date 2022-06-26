use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};
use tokio_postgres::{Client, Config, NoTls};
use tracing::info;
// 定义传入参数格式，sql执行逻辑
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Article {
    pub id: Option<i32>,
    pub title: String,
    pub descriptions: String,
}

#[derive(Deserialize, Debug)]
pub struct Pgconfig {
    pub postgres: HashMap<String, String>,
}

impl Article {
    pub fn new(id: Option<i32>, title: &str, descriptions: &str) -> Self {
        Article {
            id: id,
            title: title.into(),
            descriptions: descriptions.into(),
        }
    }
}

pub struct PostgresLogin {
    pub ip: String,
    pub user: String,
    pub password: String,
    pub db: String,
    pub port: u16,
}

///启动程序的时候执行一次，创表语句
pub async fn create_table(client: &Client, tablename: &str) -> Result<(), Error> {
    let create_table_sql = format!(
        "CREATE TABLE IF NOT EXISTS {} (
        id                 SERIAL,
        title              VARCHAR,
        descriptions        VARCHAR,
        tokens             tsvector
        )",
        tablename
    );
    client.batch_execute(&create_table_sql).await?;
    Ok(())
}

//distinct on (descriptions)，对内容进行去重，定时任务，每天凌晨执行一次
pub async fn distinct_pgsql(
    client: &Client,
    tablename: &str,
    distinct_str: &str,
) -> Result<(), Error> {
    let distinct_sql = format!(
        "delete from {} t1 where  exists (select 1 from {} t2 where t1.descriptions=t2.descriptions and t1.id<t2.id);",
        tablename, tablename
    );
    let rows = client.execute(&distinct_sql, &[]).await?;
    info!("distinct rows : {:?}", rows);
    Ok(())
}
