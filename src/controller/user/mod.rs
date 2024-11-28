pub mod route;
pub mod select;
pub mod update;
pub mod ws;

use serde::Serialize;
use sqlx::FromRow;

// 用户信息返回
#[derive(Debug, Serialize, FromRow)]
pub struct UserInfoResp {
    pub fullname: String,
    pub avatar: String,
}
