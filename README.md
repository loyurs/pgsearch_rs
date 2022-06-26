# rust+postgres搜索能力

**功能**

**1.将json等结构化数据批量导入pg,本地导入(no)**

**2.通过接口将数据导入到pg并可以进行查询(yes)**

## 1. Postgres使用方式简明介绍

**i>通过docker运行一个postgresql**

**docker run -d --name psql --restart always -p 5432:5432 postgres**

**ii>将postgres配置写入info.toml，将使用该文件连接数据库**

```
[postgres]
user = "xx"
password = "xx"
host = "xx"
port = "xx"
database = "xx"
tablename = "xx"
pool_nums = "10"
```

**iii>如果是首次运行数据库**

```
docker exec -ti postgres bash
su - postgres
psql
CREATE DATABASE library;#创建数据库
\c library   #连接数据库
\dt #查看表
```

**iv>创建**
**create extension pg_trgm;**

**v>创建索引**

**CREATE INDEX IF NOT EXISTS token_idx ON search_tables USING GIN(tokens);**

**vi>>搜索语句(示例)**

**SELECT id, title, ts_rank(tokens, query) AS score FROM searchtable, to_tsquery('simple', '人民') query WHERE tokens @@ query ORDER BY score DESC;**

**| 表示或& 表示 且，搜索采用&**

## 2. 程序步骤

**i>创建一张具有自增id的表格**

**ii>update将文章结构通过api进行存入/insert将数据插入**

**创建表语句**

```
 "CREATE TABLE IF NOT EXISTS search_tables (
        id                 SERIAL,
        title              VARCHAR,
        descriptions        VARCHAR,
        tokens             tsvector
        )"
```

**iii>查询**

```
SELECT id, title, ts_rank(tokens, query) AS score FROM search_tables, to_tsquery('simple', '人民') query WHERE tokens @@ query ORDER BY score DESC;

```

**iv>去重**

```

delete from search_tables t1 where  exists (select 1 from search_tables t2 where t1.descriptions=t2.descriptions and t1.id<t2.id);
```

### 2.1程序接口

**插入数据**

[http://ip:port/insert](http://ip:port/insert)

```
{"title":"今天是周日","descriptions":"周日是双休的一天"}
```

**查询数据**

[http://ip:port/search](http://ip:port/search)

```
{"words":"周日"}
```


**参考资料：**

**postgres：**[https://www.postgresql.org/docs/current/textsearch.html](https://www.postgresql.org/docs/current/textsearch.html)
