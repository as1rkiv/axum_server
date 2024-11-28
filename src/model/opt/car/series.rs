use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 车辆品牌系列表名
pub const TABLE_CAR_SERIES: &str = "opt_car_series";

// 车辆品牌系列
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OptCarSeries {
    pub id: i64,
    // 品牌ID
    pub brand_id: i64,
    // 品牌名称
    pub brand_name: String,
    // 系列名
    pub name: String,
    // 系列logo
    pub logo: String,
    // 系列等级
    pub level: String,
    // 制造厂
    pub factory: String,
    // 动力来源
    pub power: String,
    // 软删除
    pub is_deleted: bool,
}
