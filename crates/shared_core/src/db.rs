use anyhow::Result;
use bbs_utils::meta::SafeMetaInfo;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Sqlite,
};
use std::time::Duration;
use tokio::fs;
//初始化一个sqlite的连接池
//默认数据库名称为leaptourbbs.db
pub async fn initial(meta: SafeMetaInfo, once_time: bool) -> Result<sqlx::Pool<Sqlite>> {
    let path = meta.data_path.join("leaptourbbs.db");
    if once_time && path.is_file() {
        fs::remove_file(&path).await.unwrap();
    }
    let busy_timeout = SqliteConnectOptions::new()
        .filename(path)
        .busy_timeout(Duration::from_millis(6000));
    let busy_timeout = busy_timeout;
    let options = busy_timeout.create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(12)
        .connect_with(options)
        .await?;
    Ok(pool)
}
