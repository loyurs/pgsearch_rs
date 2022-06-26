use anyhow::Error;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use deadpool_postgres::Pool;
use jieba_rs::Jieba;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

use crate::psql::Article;

async fn health_check() -> &'static str {
    info!("hahaha");
    "Healthy"
}

#[derive(Deserialize, Serialize, Debug)]
struct SearchedMsg {
    title: String,
    description: String,
    score: f32,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct SearchWords {
    words: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct ResultJson {
    rows: Vec<SearchedMsg>,
}

///axum函数，从postgres中查询数据，返回json格式的数据
/// 默认限制返回为10行
pub async fn query_from_postgres(
    Json(pall): Json<SearchWords>,
    Extension(pool): Extension<Arc<Pool>>,
    Extension(jeb): Extension<Arc<Jieba>>,
) -> impl IntoResponse {
    let p = tokio::spawn(async move {
        info!("Starting query");
        let search_words = jeb.cut(pall.words.as_str(), false).join(" & ");
        let client = pool.get().await.unwrap();
        let rows = client.query("SELECT DISTINCT ON (descriptions) title,descriptions, ts_rank(tokens, query) AS score FROM search_tables, to_tsquery('simple', $1) query WHERE tokens @@ query ORDER BY score DESC LIMIT 10", &[&search_words]).await.unwrap();
        //获取到一个字典结构
        //将每一行，替换为json格式，然后返回给前端
        info!("{:?}", rows.len());
        let return_msg = ResultJson {
            rows: rows
                .iter()
                .map(|row| SearchedMsg {
                    title: row.get("title"),
                    description: row.get("descriptions"),
                    score: row.get("score"),
                })
                .collect(),
        };
        return_msg
    }).await.unwrap();
    (StatusCode::ACCEPTED, Json(p))
    // for row in rows {
    //     let title_: String = row.get("title");
    //     let description_: String = row.get("descriptions");
    //     let score_: f64 = row.get("score");
    //     let search_msg = SearchedMsg {
    //         title: title_,
    //         description: description_,
    //         score: score_,
    //     };
    // }
}

///忽略重复，直接处理数据
pub async fn msg_insert_into_db(
    Json(article): Json<Article>,
    Extension(pool): Extension<Arc<Pool>>,
    Extension(jeb): Extension<Arc<Jieba>>,
) -> impl IntoResponse {
    tokio::spawn(async move {
        let client = pool.get().await.unwrap();
        let title = article.title;
        let description = article.descriptions;
        let title_search = jeb.cut(title.as_str(), false).join(" & ");
        let description_search = jeb.cut(description.as_str(), false).join(" & ");
        let insert_sql = "INSERT INTO search_tables (title,descriptions,tokens) VALUES ($1,$2,setweight(to_tsvector('simple', $3), 'A')||setweight(to_tsvector('simple', $4), 'B'))";
        let res = client
            .execute(
                insert_sql,
                &[&title, &description, &title_search, &description_search],
            )
            .await
            .unwrap();
        if res == 1u64 {
            info!("Insert success");
        } else {
            info!("Insert failed");
        }
    });
}
