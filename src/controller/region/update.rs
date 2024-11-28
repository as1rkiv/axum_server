use super::{InsertOrUpdateOptRegion, REGION_CACHE_KEY};
use crate::{
    common::error::Error, model::opt::region::TABLE_OPT_REGION, state::AppState,
    utils::recursion::Recursion,
};
use redis::AsyncCommands;

// 更新区域
pub async fn update_region_by_id(
    app_state: &AppState,
    region_id: i64,
    data: InsertOrUpdateOptRegion,
) -> Result<u64, Error> {
    // 开启事务
    let mut tx = app_state.mysql().begin().await?;

    // 检测重复编码
    let exists =
        format!("SELECT `name` FROM `{TABLE_OPT_REGION}` WHERE `is_deleted` = 0 AND `code` = ? AND `id` <> ?");
    if let Some(name) = sqlx::query_scalar::<_, String>(&exists)
        .bind(data.code.trim())
        .bind(region_id)
        .fetch_optional(&mut *tx)
        .await?
    {
        return Err(Error::Tips(format!("行政区域代码重复: {}", name)));
    }

    // 如果有 pid，检查循环依赖
    if let Some(new_pid) = data.pid {
        if Recursion::has_circular_dependency(&mut tx, TABLE_OPT_REGION, region_id, new_pid).await?
        {
            tx.rollback().await?; // 回滚事务
            return Err(Error::Tips("检测到循环依赖".into()));
        }
    }

    // 更新部门
    let query = format!(
        r#"UPDATE `{TABLE_OPT_REGION}` SET 
        `pid` = ?, `name` = ?, `code` = ? 
        WHERE `id` = ?"#
    );
    let result = sqlx::query(&query)
        .bind(data.pid)
        .bind(data.name.trim())
        .bind(data.code.trim())
        .bind(region_id)
        .execute(&mut *tx)
        .await?;

    // 提交事务
    tx.commit().await?;

    // 删除redis缓存
    app_state
        .redis_privilege()
        .get()
        .await?
        .del::<_, ()>(REGION_CACHE_KEY)
        .await
        .map_err(|e| tracing::error!("Redis 错误: {}", e))
        .ok();

    // 返回影响行数
    Ok(result.rows_affected())
}
