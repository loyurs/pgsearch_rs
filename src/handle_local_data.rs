///将获取的json文件或者字符串进行多线程切割
/// 将json文件的内容直接导入到数据库中；
use jieba_rs::Jieba;
use serde_json::{self, Value};
use std::{fs, sync::Arc};

#[cfg(feature = "localdata")]
pub async fn handle_json_file() {
    let json_str = fs::read_to_string("movies.json").unwrap();
    let json_value: Value = serde_json::from_str(json_str.as_str()).unwrap();
    // let client = psql::PostgresLogin::db_login().await.unwrap();
    let had_value = Arc::new(json_value);
    let jeb_arc = Arc::new(Jieba::new());
    let threadpools = ThreadPool::new(10);
    let (send, recv) = unbounded();
    // let send_arc = Arc::new(send);
    // let mut collects = Arc::new(Mutex::new(HashMap::new()));
    for idx in 1..1000 {
        println!("开始线程{}", idx);
        let myvalue = Arc::clone(&had_value);
        let jeb = Arc::clone(&jeb_arc);
        let (s1, _r1) = (send.clone(), recv.clone());
        let job = move || {
            let title = myvalue[idx].get("title").unwrap().to_string();
            let descriptions = myvalue[idx].get("overview").unwrap().to_string();
            let title_ = jeb.cut_for_search(title.as_str(), false).join(" ");
            let descriptions_ = jeb.cut_for_search(descriptions.as_str(), false).join(" ");
            s1.send(
                title
                    + "#@#"
                    + descriptions.as_str()
                    + "#@#"
                    + title_.as_str()
                    + "#@#"
                    + descriptions_.as_str(),
            )
            .unwrap();
            println!("结束线程{}", idx);
        };

        threadpools.execute(job);
    }
    threadpools.join();

    tokio::spawn(async move {
        let client = psql::PostgresLogin::db_login().await.unwrap();
        let pcpc: Vec<_> = recv.try_iter().collect();
        for ii in pcpc {
            let mut art = psql::Article::default();
            let pdpd: Vec<_> = ii.split("#@#").collect();

            let title = pdpd[0].to_string();
            let description = pdpd[1].to_string();
            art.title = pdpd[2].to_string();
            art.descriptions = pdpd[3].to_string();
            psql::depreciation_update_search_msg(
                title,
                description,
                &client,
                art,
                "search_tables".into(),
            )
            .await
            .unwrap();
        }
    })
    .await
    .unwrap();
}

#[cfg(feature = "localdata")]
#[test]
fn test_handle_json_file() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(handle_json_file());
}

// #[test]
//测试多线程ubound通道
// fn test_thread_pools() {
//     let tp = ThreadPool::new(2);
//     let (send, recv) = unbounded();

//     let job = || {
//         println!("123");
//         let pp = 1;
//         send.send(pp);
//         thread::sleep(Duration::from_secs(5))
//     };
//     for i in 1..10 {
//         tp.execute(job)
//     }
//     thread::spawn(|| {
//         let kk = recv.recv();
//         println!("{:?}", kk);
//     });

//     tp.join();
// }

// #[test]
//测试多生产者单消费者，线程池异步等
// fn test_opoo() {
// let (s, r) = unbounded();// }
