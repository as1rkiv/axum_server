use super::{InsertOrUpdateOptRegion, REGION_CACHE_KEY};
use crate::{common::error::Error, model::opt::region::TABLE_OPT_REGION, state::AppState};
use redis::AsyncCommands;

// 添加区域
pub async fn insert_region(
    app_state: &AppState,
    data: InsertOrUpdateOptRegion,
) -> Result<u64, Error> {
    // 检测重复编码
    let exists =
        format!("SELECT `name` FROM `{TABLE_OPT_REGION}` WHERE `is_deleted` = 0 AND `code` = ?");
    if let Some(name) = sqlx::query_scalar::<_, String>(&exists)
        .bind(data.code.trim())
        .fetch_optional(app_state.mysql())
        .await?
    {
        return Err(Error::Tips(format!("行政区域代码重复: {}", name)));
    }

    // 存储数据
    let query = format!(
        r#"INSERT INTO `{TABLE_OPT_REGION}` ( 
            `pid`, `name`, `code` 
        ) VALUES (?, ?, ?)"#
    );
    let result = sqlx::query(&query)
        .bind(data.pid)
        .bind(data.name.trim())
        .bind(data.code.trim())
        .execute(app_state.mysql())
        .await?;

    // 删除redis缓存
    app_state
        .redis_privilege()
        .get()
        .await?
        .del::<_, ()>(REGION_CACHE_KEY)
        .await
        .map_err(|e| tracing::error!("Redis 错误: {}", e))
        .ok();

    Ok(result.rows_affected())
}
