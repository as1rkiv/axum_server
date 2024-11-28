use crate::{common::error::Error, model::sys::user::TABLE_SYS_USER};
use chrono::{DateTime, Utc};
use sqlx::mysql::MySqlPool;

// 用户修改密码
pub async fn update_user_password_by_userid(
    db: &MySqlPool,
    userid: i64,
    password: String,
) -> Result<u64, Error> {
    let query = format!(r#"UPDATE `{TABLE_SYS_USER}` SET `password` = ? WHERE `id` = ?"#);
    let result = sqlx::query(&query)
        .bind(password.trim())
        .bind(userid)
        .execute(db)
        .await?;

    Ok(result.rows_affected())
}

// 更新登录IP和时间
pub async fn update_user_last_login_by_userid(
    db: &MySqlPool,
    userid: i64,
    login_ip: String,
    login_at: DateTime<Utc>,
) -> Result<u64, Error> {
    let query = format!(
        r#"UPDATE `{TABLE_SYS_USER}` SET `last_login_ip` = ?, `last_login_at` = ? WHERE `id` = ?"#
    );
    let result = sqlx::query(&query)
        .bind(login_ip)
        .bind(login_at)
        .bind(userid)
        .execute(db)
        .await?;

    Ok(result.rows_affected())
}
