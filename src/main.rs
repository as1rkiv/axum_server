mod _init;
mod common;
mod config;
mod constants;
mod controller;
mod middleware;
mod model;
mod route;
mod state;
mod store;
mod utils;

use std::{
    error::Error,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
};
use tokio::{
    net::TcpListener,
    signal::{self, unix},
};

#[tokio::main]
async fn main() {
    // 初始化全局日志收集
    let _trace = common::trace::init().await;

    // 加载全局配置
    let conf = config::get_config().await;

    // 参数重置应用
    if let Err(e) = _init::Args::from_env().exec().await {
        tracing::error!("重置失败: {e}");
        return; // 直接退出
    };

    // 监听服务任务
    common::start_message(conf.server.get_port());
    match tokio::spawn(start_main_service(conf)).await {
        Ok(res) => match res {
            Ok(s) => common::end_message(s).await,
            Err(e) => tracing::error!("启动失败: {e}"),
        },
        Err(p) => tracing::error!("服务崩溃: {p}"),
    }
}

// 主服务
async fn start_main_service(
    conf: &config::Config,
) -> Result<Arc<AtomicI32>, Box<dyn Error + Send + Sync>> {
    // 共享状态
    let stat = state::AppState::build().await?;

    // 路由
    let router = route::init(stat).await;

    // 中间件
    let service = middleware::init(router).await;

    // 监听端口
    let listener = TcpListener::bind(conf.server.get_addr()).await?;

    // 运行服务
    let signal = Arc::new(AtomicI32::new(0));
    axum::serve(listener, service)
        .with_graceful_shutdown(shutdown(Arc::clone(&signal)))
        // .tcp_nodelay(true)
        .await?;

    // 结束信号
    Ok(signal)
}

// 优雅关机
async fn shutdown(sig: Arc<AtomicI32>) {
    let shutdown_signal = async {
        signal::ctrl_c().await.expect("无法处理 SIGINT");
        libc::SIGINT
    };

    // Unix-like
    #[cfg(unix)]
    let terminate_signal = async {
        let term_signal = unix::SignalKind::terminate();
        unix::signal(term_signal)
            .expect("无法处理 SIGTERM")
            .recv()
            .await;
        libc::SIGTERM
    };

    // Windows
    #[cfg(not(unix))]
    let terminate_signal = std::future::pending::<i32>();

    let signal = tokio::select! {
        sig = shutdown_signal => sig,
        sig = terminate_signal => sig,
    };

    // 写入信号
    sig.store(signal, Ordering::Relaxed);
}
