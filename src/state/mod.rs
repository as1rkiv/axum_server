mod task;
pub mod user;
pub mod websocket;

use crate::{
    common::error::Error,
    config,
    constants::{REDIS_INDEX_DATA, REDIS_INDEX_ENFORCER, WEBSOCKET_MQ_EXCHANGE},
    store::{
        mysql::new_mysql_pool,
        rabbitmq::RabbitMQManager,
        redis::{new_redis_pool, RedisManager},
    },
};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use sqlx::MySqlPool;
use std::sync::Arc;
use websocket::WebSocketManager;

// 导出封装类型
#[derive(Clone)]
pub struct AppState(pub(self) Arc<StateInner>);

// 应用共享状态
// 当前实现不可避免会产生交叉引用
// 后期应考虑拆分微服务解耦
struct StateInner {
    mysql: MySqlPool,
    redis: RedisManager,
    rabbitmq: RabbitMQManager,
    ws: WebSocketManager,
}

/*
**  公共方法
*/

impl AppState {
    /// 根据配置构建 app_tate
    pub async fn build() -> Result<Self, Error> {
        // 加载配置
        let config = config::get_config().await;

        // MySql
        let mysql = new_mysql_pool(config.mysql.get_conn_url().as_str()).await?;

        // Redis
        let redis = RedisManager {
            privilege: new_redis_pool(REDIS_INDEX_ENFORCER).await?,
            data: new_redis_pool(REDIS_INDEX_DATA).await?,
        };

        // RabbitMQ
        let rabbitmq = RabbitMQManager::build().await?;

        // WebSocket
        let ch = rabbitmq
            .get_declared_ex_channel(WEBSOCKET_MQ_EXCHANGE)
            .await?;
        let ws = WebSocketManager::new(ch).await;

        // 创建状态
        let state = Self(Arc::new(StateInner {
            mysql,
            redis,
            rabbitmq,
            ws,
        }));

        // 开始后台任务
        state.start_task().await?;

        Ok(state)
    }

    /// mysql 连接池
    pub fn mysql(&self) -> &MySqlPool {
        &self.0.mysql
    }

    /// redis 权限库
    pub fn redis_privilege(&self) -> &Pool<RedisConnectionManager> {
        &self.0.redis.privilege
    }

    /// redis 数据缓存库
    pub fn redis_data(&self) -> &Pool<RedisConnectionManager> {
        &self.0.redis.data
    }

    /// 消息队列
    pub fn rabbitmq(&self) -> &RabbitMQManager {
        &self.0.rabbitmq
    }

    /// WebSocket
    pub fn websocket(&self) -> &WebSocketManager {
        &self.0.ws
    }
}

/*
**  私有方法
*/

impl AppState {
    // 后台任务
    async fn start_task(&self) -> Result<(), Error> {
        // 消息队列存储
        self.task_websocket_message_store().await?;

        // 消息队列推送
        self.task_websocket_message_push().await?;

        Ok(())
    }
}
