use super::UserInfoResp;
use crate::{
    common::error::Error,
    model::sys::user::{User, TABLE_SYS_USER},
};
use sqlx::mysql::MySqlPool;

// 根据用户名查找用户
pub async fn query_user_by_username(db: &MySqlPool, username: &str) -> Result<Option<User>, Error> {
    let query =
        format!(r#"SELECT * FROM `{TABLE_SYS_USER}` WHERE `is_deleted` = 0 AND `username` = ?"#);
    let row: Option<User> = sqlx::query_as(&query)
        .bind(username.trim())
        .fetch_optional(db)
        .await?;

    Ok(row)
}

// 根据用户id查找用户
pub async fn query_user_by_userid(db: &MySqlPool, userid: i64) -> Result<Option<User>, Error> {
    let query = format!(r#"SELECT * FROM `{TABLE_SYS_USER}` WHERE `is_deleted` = 0 AND `id` = ?"#);
    let row: Option<User> = sqlx::query_as(&query)
        .bind(userid)
        .fetch_optional(db)
        .await?;

    Ok(row)
}

// 根据用户id查找用户信息
pub async fn query_user_info_by_userid(
    db: &MySqlPool,
    userid: i64,
) -> Result<Option<UserInfoResp>, Error> {
    let query = format!(
        r#"SELECT `fullname`, `avatar` FROM `{TABLE_SYS_USER}` WHERE `is_deleted` = 0 AND `id` = ?"#
    );
    let row: Option<UserInfoResp> = sqlx::query_as(&query)
        .bind(userid)
        .fetch_optional(db)
        .await?;

    Ok(row)
}
