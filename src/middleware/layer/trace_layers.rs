use crate::common::{self, trace::X_REQUEST_ID};
use axum::{http::header::AUTHORIZATION, Router};
use std::iter::once;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    trace::TraceLayer,
};

/// 追踪层
pub trait TraceLayersExt {
    fn with_trace_layers(self) -> Self;
}

impl TraceLayersExt for Router {
    fn with_trace_layers(self) -> Self {
        // 生成id
        let set_request_id_layer = SetRequestIdLayer::new(X_REQUEST_ID.clone(), MakeRequestUuid);

        // 传播id
        let propagate_request_id_layer = PropagateRequestIdLayer::new(X_REQUEST_ID);

        // 敏感头
        let sensitive_layer = SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION));

        // 链路追踪
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(common::trace::Formatter)
            .on_request(common::trace::Formatter)
            .on_response(common::trace::Formatter);

        self.layer(trace_layer)
            .layer(sensitive_layer)
            .layer(propagate_request_id_layer)
            .layer(set_request_id_layer)
    }
}
