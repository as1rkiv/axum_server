pub mod jwt;
mod layer;
pub mod privilege;

use axum::{extract::connect_info::IntoMakeServiceWithConnectInfo, Router};
use layer::{ErrorLayerExt, HttpLayersExt, LimitLayersExt, TraceLayersExt};
use std::net::SocketAddr;

// 初始化中间件
pub async fn init(router: Router) -> IntoMakeServiceWithConnectInfo<Router, SocketAddr> {
    /*
     **     !!! 中间件顺序极为重要 !!!
     **     router 直接通过 layer 添加时，router 为最内层，service 为最外层
     **     使用 tower::ServiceBuilder 添加时，上层为外层，下层为内层
     **     两种方式相反
     */

    router
        .with_http_layers() // 压缩 ⬆ ⬇
        .with_limit_layers() // 跨域 超时 上传限制 ⬆ ⬇
        .with_trace_layers() // 链路追踪 敏感头 传播id 生成id ⬆ ⬇
        .with_error_layers() // 错误捕捉 正常响应 ⬆ ⬇
        .into_make_service_with_connect_info::<SocketAddr>() // 请求 响应
}
