use crate::{common, config};
use bb8::{Pool, RunError};
use bb8_redis::{bb8, RedisConnectionManager};
use redis::RedisError;

/// Redis 分库管理
#[derive(Debug, Clone)]
pub struct RedisManager {
    pub privilege: Pool<RedisConnectionManager>,
    pub data: Pool<RedisConnectionManager>,
}

// 创建redis连接池
pub async fn new_redis_pool(index: u32) -> Result<Pool<RedisConnectionManager>, RedisError> {
    // 获取redis配置
    let conf = &config::get_config().await.redis;

    // 创建redis连接
    let manager = RedisConnectionManager::new(conf.get_conn_url(index))?;

    bb8::Pool::builder().build(manager).await
}

// 从 bb8::RunError 转换为自定义 Error
impl From<RunError<RedisError>> for common::error::Error {
    fn from(error: RunError<RedisError>) -> Self {
        tracing::error!("BB8 错误: {}", error);

        Self::Unavailable
    }
}

// 从 redis::RedisError 转换为自定义 Error
impl From<RedisError> for common::error::Error {
    fn from(error: RedisError) -> Self {
        tracing::error!("Redis 错误: {}", error);

        Self::Unavailable
    }
}
