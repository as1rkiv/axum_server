use crate::{common::error::Error, constants::UNKNOWN_PANIC_INFO};
use axum::{response::IntoResponse, Router};
use tower_http::catch_panic::CatchPanicLayer;

/// 错误层
pub trait ErrorLayerExt {
    fn with_error_layers(self) -> Self;
}

impl ErrorLayerExt for Router {
    fn with_error_layers(self) -> Self {
        // 响应panic
        type PanicError = Box<dyn std::any::Any + Send + 'static>;
        let panic_layer = CatchPanicLayer::custom(|err: PanicError| {
            // 解析错误信息
            if let Some(s) = err.downcast_ref::<String>() {
                tracing::error!("服务崩溃: {}", s);
            } else if let Some(s) = err.downcast_ref::<&str>() {
                tracing::error!("服务崩溃: {}", s);
            } else {
                tracing::error!("服务崩溃: {}", UNKNOWN_PANIC_INFO);
            };

            // 响应错误
            Error::Unavailable.into_response()
        });

        self.layer(panic_layer)
    }
}
