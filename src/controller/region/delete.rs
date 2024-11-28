use super::REGION_CACHE_KEY;
use crate::{common::error::Error, model::opt::region::TABLE_OPT_REGION, state::AppState};
use redis::AsyncCommands;

// 软删除地区
pub async fn delete_region_by_id(app_state: &AppState, region_id: i64) -> Result<u64, Error> {
    let query =
        format!(r#"UPDATE `{TABLE_OPT_REGION}` SET `is_deleted` = 1 WHERE `id` = ? OR `pid` = ?"#);
    let result = sqlx::query(&query)
        .bind(region_id)
        .bind(region_id)
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
