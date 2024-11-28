pub mod error;
pub mod request;
pub mod response;
pub mod trace;

use crate::{constants::SERVER_NAME, utils::fmt::BoxedMessage};
use chrono::Local;
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Arc,
};

// BoxedMessage
const BOXED_MESSAGE_MARGIN_TB: usize = 1;
const BOXED_MESSAGE_MARGIN_LEFT: usize = 4;
const BOXED_MESSAGE_PADDING_TB: usize = 1;
const BOXED_MESSAGE_PADDING_LR: usize = 8;

// 服务启动信息
pub fn start_message(port: i64) {
    let formatted_time = Local::now()
        .format("\x1b[34m%Y-%m-%d %H:%M:%S\x1b[0m")
        .to_string();

    let start = format!("\x1b[32m服务启动 {port}\x1b[0m");

    BoxedMessage::new("*")
        .set_margin(BOXED_MESSAGE_MARGIN_TB, BOXED_MESSAGE_MARGIN_LEFT)
        .set_padding(BOXED_MESSAGE_PADDING_TB, BOXED_MESSAGE_PADDING_LR)
        .append(SERVER_NAME)
        .append(&start)
        .append(&formatted_time)
        .show_message();
}

// 服务关闭信息
pub async fn end_message(signal: Arc<AtomicI32>) {
    // 格式化时间
    let formatted_time = Local::now()
        .format("\x1b[34m%Y-%m-%d %H:%M:%S\x1b[0m")
        .to_string();

    // 解锁匹配
    let end = {
        let sig_str = match signal.load(Ordering::Relaxed) {
            libc::SIGINT => "SIGINT",
            libc::SIGTERM => "SIGTERM",
            _ => "UNKNOWN",
        };
        format!("\x1b[31m服务终止 {sig_str}\x1b[0m")
    };

    BoxedMessage::new("*")
        .set_margin(BOXED_MESSAGE_MARGIN_TB, BOXED_MESSAGE_MARGIN_LEFT)
        .set_padding(BOXED_MESSAGE_PADDING_TB, BOXED_MESSAGE_PADDING_LR)
        .append(SERVER_NAME)
        .append(&end)
        .append(&formatted_time)
        .show_message();
}
