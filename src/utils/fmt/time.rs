use chrono::Utc;

/// 格式化延迟时间，输入为 `Option<i64>` 纳秒，返回 (延迟值, 单位)
pub fn format_latency(start_nanosec: Option<i64>) -> String {
    // 检查输入的纳秒时间戳是否为 Some，并获取当前时间的纳秒时间戳
    let latency_ns = start_nanosec.and_then(|start_ns| {
        Utc::now()
            .timestamp_nanos_opt()
            .map(|now_ns| now_ns - start_ns)
    });

    let (latency, units) = match latency_ns {
        // 超过 1,000 毫秒，转换为秒
        Some(ns) if ns >= 1_000_000_000 => (ns as f64 / 1_000_000_000.0, r#"s"#),
        // 超过 1,000 微秒，转换为毫秒
        Some(ns) if ns >= 1_000_000 => (ns as f64 / 1_000_000.0, r#"ms"#),
        // 超过 1,000 纳秒，转换为微秒
        Some(ns) if ns >= 1_000 => (ns as f64 / 1_000.0, r#"μs"#),
        // 小于 1,000 纳秒，保持纳秒
        Some(ns) => (ns as f64, r#"ns"#),
        // 输入为 None 或获取当前时间失败，返回默认值
        None => (0.0, r#"ns"#),
    };

    format!(r#"{latency:.2}{units}"#)
}
