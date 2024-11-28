use super::NewMessage;
use axum::{
    extract::ws::{Message, WebSocket},
    Error,
};
use futures::{sink::SinkExt, stream::SplitSink};
use std::{future::Future, ops::ControlFlow, sync::Arc};
use tokio::sync::Mutex;

// WebSocket 连接
#[derive(Debug)]
pub struct WebSocketConnection(Arc<Mutex<SplitSink<WebSocket, Message>>>);

impl WebSocketConnection {
    pub fn new(sender: SplitSink<WebSocket, Message>) -> Self {
        Self(Arc::new(Mutex::new(sender)))
    }

    // 发送二进制消息
    pub async fn send_message(&self, vec: Vec<u8>) -> Result<(), Error> {
        self.0.lock().await.send(Message::Binary(vec)).await
    }

    // 处理接收到的消息
    pub async fn proc_all_msg<F, Fut>(
        message: Result<Message, Error>,
        handler: F,
    ) -> ControlFlow<(), ()>
    where
        F: Fn(NewMessage) -> Fut + Sync + Send,
        Fut: Future<Output = Result<(), ()>> + Send,
    {
        // 消息完整性
        let msg = match message {
            Ok(msg) => msg,
            Err(err) => {
                tracing::error!("WebSocket 接收失败: {}", err);
                return ControlFlow::Break(());
            }
        };

        // 处理消息内容
        match msg {
            // 二进制消息
            Message::Binary(binary) => Self::proc_binary(binary, handler).await,
            // 文本消息
            Message::Text(text) => Self::proc_text(text),
            // 心跳，自动处理，不干预
            Message::Ping(_) | Message::Pong(_) => ControlFlow::Continue(()),
            // 关闭连接
            Message::Close(_) => ControlFlow::Break(()),
        }
    }

    // 二进制消息处理
    async fn proc_binary<F, Fut>(binary: Vec<u8>, handler: F) -> ControlFlow<(), ()>
    where
        F: Fn(NewMessage) -> Fut + Sync + Send,
        Fut: Future<Output = Result<(), ()>> + Send,
    {
        match NewMessage::new(binary) {
            Ok(msg) => {
                // 成功反序列化, 调用 handler 处理
                if handler(msg).await.is_err() {
                    return ControlFlow::Break(());
                };
                ControlFlow::Continue(())
            }
            Err(err) => {
                tracing::error!("WebSocket 反序列化失败: {}", err);
                ControlFlow::Break(())
            }
        }
    }

    // 文本消息处理
    fn proc_text(text: String) -> ControlFlow<(), ()> {
        tracing::error!("WebSocket 非法内容: {}", text);
        ControlFlow::Break(())
    }
}
