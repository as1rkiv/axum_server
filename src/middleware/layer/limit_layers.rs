use crate::constants::{MAX_TIMEOUT, MAX_UPLOAD_SIZE};
use axum::{extract::DefaultBodyLimit, Router};
use std::time::Duration;
use tower_http::{
    cors::{self, CorsLayer},
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
};

/// 限制层
pub trait LimitLayersExt {
    fn with_limit_layers(self) -> Self;
}

impl LimitLayersExt for Router {
    fn with_limit_layers(self) -> Self {
        // 强制执行请求体大小限制
        let request_body_limit_layer = RequestBodyLimitLayer::new(MAX_UPLOAD_SIZE);

        // 默认请求体大小限制
        let default_body_limit_layer = DefaultBodyLimit::disable();

        // 超时
        let timeout_layer = TimeoutLayer::new(Duration::from_secs(MAX_TIMEOUT));

        // 跨域
        let cors_layer = CorsLayer::new()
            .allow_origin(cors::Any)
            .allow_methods(cors::Any)
            .allow_headers(cors::Any);

        self.layer(cors_layer)
            .layer(timeout_layer)
            .layer(default_body_limit_layer)
            .layer(request_body_limit_layer)
    }
}
