use axum::{
    extract::ConnectInfo,
    http::{Request, Response},
};
use chrono::Local;
use std::net::SocketAddr;
use tokio::time::Duration;
use tower_http::trace::{MakeSpan, OnRequest, OnResponse};
use tracing::Span;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

#[derive(Clone, Debug)]
pub struct Formatter;

// 跨度
impl<B> MakeSpan<B> for Formatter {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        // 获取请求的IP地址
        let remote_addr = request
            .extensions()
            .get::<ConnectInfo<SocketAddr>>() // 从请求扩展中获取 SocketAddr
            .map(|ConnectInfo(addr)| addr.to_string()) // 将 SocketAddr 转换为字符串
            .unwrap_or_else(|| "Unknown".to_string());

        // 自定义 http 的输出格式
        tracing::span!(
            tracing::Level::ERROR,
            "Http",
            method = %request.method(),
            uri = %request.uri(),
            client_ip = %remote_addr,
            version = ?request.version(),
            headers = ?request.headers()
        )
    }
}

// 请求
impl<B> OnRequest<B> for Formatter {
    fn on_request(&mut self, _: &Request<B>, _: &Span) {}
}

// 响应
impl<B> OnResponse<B> for Formatter {
    fn on_response(self, response: &Response<B>, latency: Duration, _: &Span) {
        tracing::info!(
            target: "http",
            status = %response.status(),
            latency = %format_args!("{:.2?}", latency)
        );
    }
}

// 时间
impl FormatTime for Formatter {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let now = Local::now();
        write!(w, "{}", now.format("[%Y-%m-%d %H:%M:%S]"))
    }
}
