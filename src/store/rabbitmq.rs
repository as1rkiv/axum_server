use crate::{common::error::Error, config};
use lapin::{
    options::{BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
    Channel, Connection, ConnectionProperties, Consumer, ExchangeKind,
};
use std::{collections::HashMap, future::Future, sync::Arc};
use tokio::{sync::Mutex, task::JoinHandle};

/// 消息队列管理
#[derive(Debug, Clone)]
pub struct RabbitMQManager {
    /// 全局连接
    conn: Arc<Connection>,
    /// 消费者
    cons: Arc<Mutex<HashMap<String, JoinHandle<()>>>>,
}

impl RabbitMQManager {
    pub async fn build() -> Result<Self, Error> {
        // 获取 rabbitmq 配置
        let conf = &config::get_config().await.rabbitmq;

        // 创建新连接
        let conn =
            Connection::connect(&conf.get_conn_url(), ConnectionProperties::default()).await?;

        Ok(Self {
            conn: Arc::new(conn),
            cons: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// 返回一个声明了交换机的新通道
    pub async fn get_declared_ex_channel(&self, exchnage: &str) -> Result<Channel, Error> {
        // 连接通道
        let ch = self.conn.create_channel().await?;

        //声明交换机
        ch.exchange_declare(
            exchnage,
            ExchangeKind::Fanout,
            ExchangeDeclareOptions {
                passive: false,     // 被动声明交换机
                durable: true,      // 交换机持久化
                auto_delete: false, // 自动删除交换机
                internal: false,    // 内部使用
                nowait: false,      // 不等待声明结果确认
            },
            FieldTable::default(),
        )
        .await?;

        Ok(ch)
    }

    /// 返回一个声明了交换机，且绑定路由和队列的消费者
    pub async fn get_bind_rq_consumer(
        &self,
        exchange: &str,
        routing_key: &str,
        queue: &str,
    ) -> Result<Consumer, Error> {
        //声明交换机
        let ch = self.get_declared_ex_channel(exchange).await?;

        //声明推送队列
        ch.queue_declare(
            queue,
            QueueDeclareOptions {
                passive: false,     // 被动声明队列
                durable: true,      // 队列持久化
                exclusive: false,   // 队列独占
                auto_delete: false, // 自动删除队列
                nowait: false,      // 不等待声明结果确认
            },
            FieldTable::default(),
        )
        .await?;

        //绑定队列
        ch.queue_bind(
            queue,
            exchange,
            routing_key,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

        // 消费者
        let consumer = ch
            .basic_consume(
                queue,
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(consumer)
    }

    /// 添加一个消费者协程
    pub async fn add_consumer<F, Fut>(&self, queue_name: &str, task: F) -> Result<(), Error>
    where
        F: FnOnce() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<JoinHandle<()>, Error>> + Send,
    {
        // 验证有效性
        let handler = task().await?;

        // 锁定插入
        let mut consumers = self.cons.lock().await;
        consumers.insert(queue_name.to_string(), handler);

        Ok(())
    }

    /// 移除一个消费者协程
    #[allow(dead_code)]
    pub async fn remove_consumer(&self, queue_name: &str) {
        if let Some(handle) = self.cons.lock().await.remove(queue_name) {
            handle.abort(); // 终止协程
        }
    }
}

// 转换自定义 Error
impl From<lapin::Error> for Error {
    fn from(error: lapin::Error) -> Self {
        tracing::error!("RabbitMQ 错误: {}", error);

        Self::Unavailable
    }
}
