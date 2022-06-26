use std::path::Path;

use anyhow::Error;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::{Client, Config, NoTls};

use crate::psql::Pgconfig;
//use info.toml as config;
pub async fn new_postgres_pool(os_path: &str) -> Result<Pool, Error> {
    let toml_content = tokio::fs::read_to_string(std::path::Path::new(os_path)).await?;
    //结构需要和info.toml中的[var]一致
    let info_config: Pgconfig = toml::from_str(&toml_content).unwrap();
    let mut config = Config::new();
    let pgcfg = config
        .host(info_config.postgres.get("host").unwrap())
        .user(info_config.postgres.get("user").unwrap())
        .dbname(info_config.postgres.get("database").unwrap())
        .password(info_config.postgres.get("password").unwrap())
        .port(
            (info_config.postgres.get("port").unwrap())
                .parse::<u16>()
                .unwrap(),
        );
    let pool_nums: usize = info_config
        .postgres
        .get("pool_nums")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pgcfg.to_owned(), NoTls, mgr_config);
    let pool = Pool::builder(mgr).max_size(pool_nums).build().unwrap();
    Ok(pool)
}

///use tokio_postgres to create a tokio_postgres pool  
pub async fn make_pool(pgcfg: Config, pool_nums: usize) -> Result<Pool, Error> {
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pgcfg, NoTls, mgr_config);
    let pool = Pool::builder(mgr).max_size(pool_nums).build().unwrap();
    Ok(pool)
}

///new client for postgres
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
