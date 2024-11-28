use super::{NewMessage, WebSocketCommand, WebSocketConnection, WebSocketMessage};
use crate::{
    constants::{WEBSOCKET_MPSC_SIZE, WEBSOCKET_MQ_EXCHANGE, WEBSOCKET_MQ_ROUTING},
    utils::fmt::format_latency,
};
use dashmap::{mapref::one::RefMut, DashMap};
use lapin::{options::BasicPublishOptions, BasicProperties, Channel};
use serde_json::to_vec;
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex, RwLock,
    },
    task::JoinHandle,
};

/// WebSocket 连接管理，单体服务下，暂用此方案
/// 支持多用户多设备消息转发
/// 如性能到达瓶颈，考虑将 WebSocket
/// 拆分至 Erlang 微服务
/// 通讯使用 redis 发布订阅 或 gPRC
#[derive(Debug, Clone)]
pub struct WebSocketManager {
    /// 消息发送端
    tx_sender: Arc<RwLock<Sender<WebSocketCommand>>>,
    /// 消息接收端任务
    rx_task: Arc<Mutex<Option<JoinHandle<()>>>>,
    /// socket连接池
    conn_pool: Arc<ConnectionPool>,
    /// rabbitmq 连接通道
    mq_ch: Arc<Channel>,
}

/// 连接池哈希表
type ConnectionPool = DashMap<i64, DashMap<i64, WebSocketConnection>>;

/*
** 公共方法
*/

impl WebSocketManager {
    /// 创建管理任务
    pub async fn new(mq_ch: Channel) -> Self {
        // 初始化管道
        let (tx, rx) = mpsc::channel(WEBSOCKET_MPSC_SIZE);

        // 启动管理任务
        let manager = Self {
            tx_sender: Arc::new(RwLock::new(tx)),
            rx_task: Arc::new(Mutex::new(None)),
            conn_pool: Arc::new(DashMap::new()),
            mq_ch: Arc::new(mq_ch),
        };

        // 启动管理任务
        manager.start_rx_task(rx).await;
        manager
    }

    /// 添加连接
    pub async fn add_conn(
        &self,
        user_id: i64,
        conn_id: i64,
        conn: WebSocketConnection,
    ) -> Result<(), ()> {
        if let Err(err) = self
            .tx_sender
            .read()
            .await
            .send(WebSocketCommand::ConnAdd {
                user_id,
                conn_id,
                conn,
            })
            .await
        {
            tracing::error!("MPSC 发送失败: {}", err);

            // 重启管理任务
            self.restart_rx_task().await;
            tracing::warn!("WebSocket 服务已重启");

            return Err(());
        }
        Ok(())
    }

    /// 移除连接
    pub async fn remove_conn(&self, user_id: i64, conn_id: i64) -> Result<(), ()> {
        if let Err(err) = self
            .tx_sender
            .read()
            .await
            .send(WebSocketCommand::ConnRemove { user_id, conn_id })
            .await
        {
            tracing::error!("MPSC 发送失败: {}", err);

            // 重启管理任务
            self.restart_rx_task().await;
            tracing::warn!("WebSocket 服务已重启");

            return Err(());
        }
        Ok(())
    }

    /// 处理连接消息
    pub async fn proc_conn(
        &self,
        user_id: i64,
        message: NewMessage,
        ipaddr: SocketAddr,
    ) -> Result<(), ()> {
        if let Err(err) = self
            .tx_sender
            .read()
            .await
            .send(WebSocketCommand::MsgProc {
                wsm: (user_id, ipaddr.ip().to_string(), message).into(),
            })
            .await
        {
            tracing::error!("MPSC 发送失败: {}", err);

            // 重启管理任务
            self.restart_rx_task().await;
            tracing::warn!("WebSocket 服务已重启");

            return Err(());
        }
        Ok(())
    }
}

/*
** 私有方法
*/

impl WebSocketManager {
    /// 开启任务
    async fn start_rx_task(&self, mut rx: Receiver<WebSocketCommand>) {
        let mq_ch = Arc::clone(&self.mq_ch);
        let conn_pool = Arc::clone(&self.conn_pool);

        // 监听mpsc消息
        // 采用协程方便中断与重启
        let task = tokio::spawn(async move {
            while let Some(command) = rx.recv().await {
                match command {
                    // 添加连接
                    WebSocketCommand::ConnAdd {
                        user_id,
                        conn_id,
                        conn,
                    } => {
                        // 添加一个连接的设备到用户下
                        conn_pool.entry(user_id).or_default().insert(conn_id, conn);
                    }

                    // 移除连接
                    WebSocketCommand::ConnRemove { user_id, conn_id } => {
                        if let Some(user_conns) = conn_pool.get_mut(&user_id) {
                            // 移除设备连接
                            Self::remove_device(&user_conns, &conn_id);
                        }
                    }

                    // 消息处理
                    WebSocketCommand::MsgProc { wsm } => {
                        // 如发布消息失败，不转发
                        if Self::publish_messages(&mq_ch, &wsm).await.is_ok() {
                            // 处理消息转发
                            Self::forward_message(&conn_pool, wsm).await;
                        };
                    }
                }
            }
        });

        // 保存任务句柄
        *self.rx_task.lock().await = Some(task);
    }

    /// 重启任务
    async fn restart_rx_task(&self) {
        // 终止旧任务
        {
            if let Some(task) = self.rx_task.lock().await.take() {
                task.abort();
            }
        } // 解锁

        // 创建新通道
        let (new_tx, new_rx) = mpsc::channel(WEBSOCKET_MPSC_SIZE);

        // 替换发送端
        {
            *self.tx_sender.write().await = new_tx;
        } // 解锁

        // 重新开启任务
        self.start_rx_task(new_rx).await;
    }

    /// 消息分发
    async fn forward_message(pool: &Arc<ConnectionPool>, wsm: WebSocketMessage) {
        let sender = wsm.get_sender();
        let receiver = wsm.get_receiver();
        let sent_time = wsm.get_sent_time().timestamp_nanos_opt();

        // 创建发送任务
        let mut to_remove = vec![];
        if let Some(user_conns) = pool.get(&receiver) {
            // 先尝试序列化消息
            match to_vec::<WebSocketMessage>(&wsm) {
                Ok(vec_msg) => {
                    // 遍历用户设备并发送消息
                    for conn in user_conns.iter() {
                        if let Err(err) = conn.send_message(vec_msg.clone()).await {
                            tracing::error!("WebSocket 分发失败: {err}");

                            // 返回发送失败的设备ID
                            to_remove.push(conn.key().to_owned());
                        } else {
                            tracing::info!(
                                "WebSocket 消息分发: from {} to {} latency {}",
                                sender,
                                receiver,
                                format_latency(sent_time)
                            );
                        }
                    }
                }
                Err(err) => {
                    tracing::error!("WebSocket 序列化失败: {err}");
                }
            }
        }

        // 写锁删除离线连接
        if !to_remove.is_empty() {
            if let Some(user_conns) = pool.get_mut(&receiver) {
                to_remove.iter().for_each(|device_id| {
                    Self::remove_device(&user_conns, device_id);
                });
            }
        }
    }

    /// 向消息队列发布
    async fn publish_messages(mq_ch: &Channel, wsm: &WebSocketMessage) -> Result<(), ()> {
        // 序列化消息
        let payload = wsm
            .to_bytes()
            .map_err(|e| tracing::error!("Serde 序列化失败: {e}"))?;

        // 持久化消息
        if let Err(err) = mq_ch
            .basic_publish(
                WEBSOCKET_MQ_EXCHANGE,
                WEBSOCKET_MQ_ROUTING,
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await
        {
            tracing::error!("WebSocket 队列发布失败: {}", err);
            return Err(());
        };

        Ok(())
    }

    // 移除离线设备
    fn remove_device(
        user_conns: &RefMut<'_, i64, DashMap<i64, WebSocketConnection>>,
        device_id: &i64,
    ) {
        if user_conns.remove(device_id).is_some() {
            tracing::info!("WebSocket 连接断开: {}", device_id);
        }
    }
}
