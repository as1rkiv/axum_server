use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 车辆品牌表名
pub const TABLE_CAR_BRAND: &str = "opt_car_brand";

// 车辆品牌
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OptCarBrand {
    pub id: i64,
    // 品牌名
    pub name: String,
    // 品牌logo
    pub logo: String,
    // 所属国家
    pub country: String,
    // 首字母
    pub firstletter: String,
    // 软删除
    pub is_deleted: bool,
}
