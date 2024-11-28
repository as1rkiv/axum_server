mod constants;
mod extensions;

use crate::{common, config};
use sqlx::{
    mysql::{MySqlPool, MySqlPoolOptions},
    Error,
};
use tokio::time::Duration;

// 导出
#[allow(unused_imports)]
pub use constants::{
    ISOLATION_READ_COMMITTED, ISOLATION_READ_UNCOMMITTED, ISOLATION_REPEATABLE_READ,
    ISOLATION_SERIALIZABLE,
};
pub use extensions::QueryBuilderExt;

// 创建数据库连接池
pub async fn new_mysql_pool(conn_url: &str) -> Result<MySqlPool, Error> {
    let conf = &config::get_config().await.mysql;

    MySqlPoolOptions::new()
        .max_connections(conf.get_max_connections())
        .min_connections(conf.get_min_connections())
        .acquire_timeout(Duration::from_secs(conf.get_acquire_timeout()))
        .connect(conn_url)
        .await
}

// 从 sqlx::Error 转换为自定义 Error
impl From<Error> for common::error::Error {
    fn from(error: Error) -> Self {
        tracing::error!("Sqlx 错误: {}", error);

        Self::Unavailable
    }
}
