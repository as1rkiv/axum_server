use super::super::{websocket::WebSocketMessage, AppState};
use crate::{
    common::error::Error,
    constants::{WEBSOCKET_MQ_EXCHANGE, WEBSOCKET_MQ_QUEUE_PUSH, WEBSOCKET_MQ_ROUTING},
};
use futures::StreamExt;
use lapin::options::BasicAckOptions;

impl AppState {
    // 消息推送
    pub async fn task_websocket_message_push(&self) -> Result<(), Error> {
        // 添加消费者
        let mq = self.rabbitmq().clone();
        self.rabbitmq()
            .add_consumer(WEBSOCKET_MQ_QUEUE_PUSH, || async move {
                // 生成消息消费者
                let mut consumer = mq
                    .get_bind_rq_consumer(
                        WEBSOCKET_MQ_EXCHANGE,
                        WEBSOCKET_MQ_ROUTING,
                        WEBSOCKET_MQ_QUEUE_PUSH,
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
                                    // 推送逻辑
                                    tracing::info!("RabbitMQ 推送消息: {msg:?}",);
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

                // 推送协程
                Ok(task)
            })
            .await?;

        Ok(())
    }
}
