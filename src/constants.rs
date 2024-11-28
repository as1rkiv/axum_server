#![allow(dead_code)]

pub const SERVER_NAME: &str = "Axum Web Server";

/*
** 文件配置
*/

pub const CONFIG_FILE: &str = "config/config.toml"; // 配置文件路径
pub const SQL_FILE_DIR: &str = "sqls"; // SQL文件夹
pub const UPLOADS_DIR: &str = "uploads"; // 上传文件夹
pub const LOG_FILE_DIR: &str = "logs"; // 日志文件夹

/*
** 中间件配置
*/

pub const REQUEST_ID_HEADER: &str = "x-request-id"; // 请求追踪id
pub const UNKNOWN_PANIC_INFO: &str = "Unknown"; // panic默认原因
pub const MAX_UPLOAD_SIZE: usize = 250 * 1024 * 1024; // 最大请求250mb
pub const MAX_TIMEOUT: u64 = 30; // 连接超时时间(s)

/*
** 请求配置
*/

pub const REQUEST_DEFAULT_INDEX: i64 = 1;
pub const REQUEST_DEFAULT_PAGESIZE: i64 = 20;

/*
** Redis 配置
*/

pub const REDIS_INDEX_ENFORCER: u32 = 0; // 角色继承链缓存
pub const REDIS_INDEX_DATA: u32 = 1; // 数据缓存

/*
** WebSocket 配置
*/

pub const WEBSOCKET_MPSC_SIZE: usize = 2048; // 通道缓冲区大小
pub const WEBSOCKET_MQ_EXCHANGE: &str = "websocket"; // 交换机
pub const WEBSOCKET_MQ_ROUTING: &str = "/message"; // 消息路由
pub const WEBSOCKET_MQ_QUEUE_STORE: &str = "store"; // 持久化队列
pub const WEBSOCKET_MQ_QUEUE_PUSH: &str = "push"; // 推送队列
