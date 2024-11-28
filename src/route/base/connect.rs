use crate::{
    middleware::jwt::token::verify,
    state::{websocket::new_connection, AppState},
};
use axum::{
    extract::{connect_info::ConnectInfo, ws::WebSocketUpgrade, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde::Deserialize;
use std::{net::SocketAddr, str};

// 请求token
#[derive(Debug, Deserialize)]
pub struct WebSocketConnectPayload {
    token: Option<String>,
}

// 创建 WebSocket 连接
pub async fn ws_connect(
    State(app_state): State<AppState>,
    ws: WebSocketUpgrade,
    Query(payload): Query<WebSocketConnectPayload>,
    ConnectInfo(ipaddr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    // 检查 token
    let token_base64 = match payload.token.filter(|t| !t.is_empty()) {
        Some(base) => URL_SAFE.decode(base).ok(),
        None => return (StatusCode::FORBIDDEN, "参数无效").into_response(),
    };

    // 解码 Base64
    let token = match token_base64 {
        Some(v) => str::from_utf8(&v).unwrap_or_default().to_string(),
        None => return (StatusCode::FORBIDDEN, "解码失败").into_response(),
    };

    // 验证 token
    if let Ok(user_state) = verify(&token).await {
        // 升级协议
        return ws.on_upgrade(move |websocket| {
            // 返回连接
            new_connection(websocket, app_state, user_state, ipaddr)
        });
    }

    (StatusCode::FORBIDDEN, "验证失败").into_response()
}
