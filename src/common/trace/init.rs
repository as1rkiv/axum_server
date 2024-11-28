use super::Formatter;
use crate::{
    config::{self, Config},
    constants::LOG_FILE_DIR,
};
use tracing::Level;
use tracing_appender::{
    non_blocking,
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{self, filter::LevelFilter, fmt, layer::SubscriberExt};

// tracing 初始化
pub async fn init() -> WorkerGuard {
    // 加载配置
    let conf = config::get_config().await;

    // 文件写入
    let file_appender = file_appender_builder(conf);
    let (non_blocking_writer, guard) = non_blocking(file_appender);
    let appender_layer = fmt::Layer::new()
        .with_ansi(false) // 禁用控制台色彩
        .with_target(false) // 不显示 target
        .with_timer(Formatter)
        .with_writer(non_blocking_writer);

    // 控制台
    let console_layer = fmt::Layer::new()
        .with_ansi(true) // 启用控制台色彩
        .with_target(false) // 不显示 target
        .with_timer(Formatter);

    // 过滤级别
    let filter = string_to_level(conf.log.get_level()); // 从配置获取日志级别
    let level_filter = LevelFilter::from_level(filter); // 转换为 `LevelFilter`

    // 注册
    let subscriber = tracing_subscriber::registry()
        .with(level_filter)
        .with(appender_layer)
        .with(console_layer);

    // 设置全局日志收集
    tracing::subscriber::set_global_default(subscriber).expect("无法创建全局日志收集");

    // 返回 Guard
    guard
}

// 日志文件配置
fn file_appender_builder(config: &Config) -> RollingFileAppender {
    // 加载日志配置
    let rotation = string_to_rotation(config.log.get_rotation());

    RollingFileAppender::builder()
        .max_log_files(config.log.get_max_files())
        .filename_suffix("log")
        .rotation(rotation)
        .build(LOG_FILE_DIR)
        .expect("无法创建日志轮转")
}

// 转换日志级别到Level枚举
fn string_to_level(s: &str) -> Level {
    match s.to_ascii_uppercase().as_str() {
        "TRACE" => Level::TRACE,
        "DEBUG" => Level::DEBUG,
        "INFO" => Level::INFO,
        "WARN" => Level::WARN,
        "ERROR" => Level::ERROR,
        _ => Level::INFO,
    }
}

// 转换日志轮转到Rotation枚举
fn string_to_rotation(s: &str) -> Rotation {
    match s.to_ascii_uppercase().as_str() {
        "NEVER" => Rotation::NEVER,
        "DAILY" => Rotation::DAILY,
        "HOURLY" => Rotation::HOURLY,
        "MINUTELY" => Rotation::MINUTELY,
        _ => Rotation::HOURLY,
    }
}
