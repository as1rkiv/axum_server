use crate::{
    common::error::Error,
    state::{user::UserState, websocket::TABLE_MESSAGE_RECORD, AppState},
};
use serde::Deserialize;
use validator::Validate;

// 标记用户已读
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateChatRead {
    #[validate(required(message = "参数无效"), range(min = 1, message = "参数无效"))]
    user: Option<i64>,
}
pub async fn update_chat_read(
    app_state: &AppState,
    user_state: &UserState,
    params: UpdateChatRead,
) -> Result<u64, Error> {
    let sender = params.user.ok_or(Error::Params("参数无效"))?;

    let query = format!(
        r#"UPDATE `{TABLE_MESSAGE_RECORD}` 
        SET `read` = ? 
        WHERE `sender` = ? AND `receiver` = ?"#
    );
    let result = sqlx::query(&query)
        .bind(true)
        .bind(sender)
        .bind(user_state.get_id())
        .execute(app_state.mysql())
        .await?;

    Ok(result.rows_affected())
}
