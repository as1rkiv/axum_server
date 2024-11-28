use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 地区编码表名
pub const TABLE_OPT_REGION: &str = "opt_region";

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OptRegion {
    pub id: i64,

    // 父地区ID
    pub pid: Option<i64>,

    // 地区名称
    pub name: String,

    // 行政编码
    pub code: String,

    // 软删除
    pub is_deleted: bool,
}
