use axum::Router;
use tower_http::compression::CompressionLayer;

/// http层
pub trait HttpLayersExt {
    fn with_http_layers(self) -> Self;
}

impl HttpLayersExt for Router {
    fn with_http_layers(self) -> Self {
        // 压缩
        let compression_layer = CompressionLayer::new();

        self.layer(compression_layer)
    }
}
