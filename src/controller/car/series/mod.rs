pub mod delete;
pub mod insert;
pub mod select;
pub mod update;
pub mod upload;

use serde::Deserialize;
use validator::Validate;

// 添加或更新车辆系列
#[derive(Debug, Deserialize, Validate)]
pub struct InsertOrUpdateCarBrandSeries {
    // 品牌ID
    #[validate(range(min = 1, message = "参数无效"))]
    brand_id: i64,

    // 系列名
    #[validate(length(min = 1, message = "参数无效"))]
    pub name: String,

    // logo
    #[validate(length(min = 1, message = "参数无效"))]
    pub logo: String,

    // 车型
    #[validate(length(min = 1, message = "参数无效"))]
    pub level: String,

    // 制造商
    #[validate(length(min = 1, message = "参数无效"))]
    pub factory: String,

    // 动力
    #[validate(length(min = 1, message = "参数无效"))]
    pub power: String,
}
