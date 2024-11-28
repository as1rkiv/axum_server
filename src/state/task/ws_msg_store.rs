use super::super::{
    websocket::{WebSocketMessage, TABLE_MESSAGE_RECORD},
    AppState,
};
use crate::{
    common::error::Error,
    constants::{WEBSOCKET_MQ_EXCHANGE, WEBSOCKET_MQ_QUEUE_STORE, WEBSOCKET_MQ_ROUTING},
};
use futures::StreamExt;
use lapin::options::BasicAckOptions;

impl AppState {
    // 消息持久化
    pub async fn task_websocket_message_store(&self) -> Result<(), Error> {
        // 消息队列存储
        let db = self.mysql().clone();
        let mq = self.rabbitmq().clone();
        self.rabbitmq()
            .add_consumer(WEBSOCKET_MQ_QUEUE_STORE, || async move {
                // 生成消息消费者
                let mut consumer = mq
                    .get_bind_rq_consumer(
                        WEBSOCKET_MQ_EXCHANGE,
                        WEBSOCKET_MQ_ROUTING,
                        WEBSOCKET_MQ_QUEUE_STORE,
                    )
                    .await?;

                // 持久化协程
                let task = tokio::spawn(async move {
                    while let Some(result) = consumer.next().await {
                        // 验证消息正确
                        let data = match result {
                            Ok(data) => Some(data),
                            Err(err) => {
                                tracing::error!("RabbitMQ 订阅消息错误: {err}");
                                None
                            }
                        };

                        if let Some(data) = data {
                            match WebSocketMessage::from_bytes(&data.data) {
                                Ok(msg) => {
                                    let query = format!(
                                        r#"INSERT INTO `{TABLE_MESSAGE_RECORD}` ( 
                                        `session`, `ident`, `sender`, `receiver`, 
                                        `msg_type`, `content`, `read`, 
                                        `sent_ip`, `sent_at` 
                                        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);"#
                                    );
                                    if let Err(err) = sqlx::query(&query)
                                        .bind(msg.session)
                                        .bind(msg.ident)
                                        .bind(msg.sender)
                                        .bind(msg.receiver)
                                        .bind(msg.msg_type as u8)
                                        .bind(msg.content)
                                        .bind(msg.read)
                                        .bind(msg.sent_ip)
                                        .bind(msg.sent_at)
                                        .execute(&db)
                                        .await
                                    {
                                        tracing::error!("RabbitMQ 持久化错误: {err}");
                                    }
                                }
                                Err(err) => {
                                    tracing::error!("RabbitMQ 反序列化错误: {err}");
                                }
                            }

                            if let Err(err) = data.ack(BasicAckOptions::default()).await {
                                tracing::error!("RabbitMQ 消息应答错误: {err}");
                            }
                        }
                    }
                });

                Ok(task)
            })
            .await?;

        Ok(())
    }
}
