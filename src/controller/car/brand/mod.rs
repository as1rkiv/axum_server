pub mod delete;
pub mod insert;
pub mod option;
pub mod select;
pub mod update;
pub mod upload;

use serde::Deserialize;
use validator::Validate;

// 添加或更新车辆品牌
#[derive(Debug, Deserialize, Validate)]
pub struct InsertOrUpdateCarBrand {
    // 品牌名
    #[validate(length(min = 1, message = "参数无效"))]
    pub name: String,

    // logo
    #[validate(length(min = 1, message = "参数无效"))]
    pub logo: String,

    // 首字母
    #[validate(length(min = 1, max = 1, message = "参数无效"))]
    pub firstletter: String,

    // 所属国家
    #[validate(length(min = 1, message = "参数无效"))]
    pub country: String,
}
