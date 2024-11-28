mod user;

use super::data::DataScope;
use crate::common::error::Error;
use bb8::PooledConnection;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::{collections::HashMap, future::Future};

/// 部门角色数据范围
const KEY_DEPT_DATASCOPE: &str = "KDS";

// 导出
pub use user::{del_user_privileges_by_id, del_users_privileges, get_user_privileges};

/// 部门角色数据范围
#[derive(Debug, Clone, Copy, Serialize, Deserialize, FromRow)]
pub struct UserDeptDataScope {
    // 部门ID
    pub dept_id: i64,
    // 角色数据范围
    pub data_scope: DataScope,
}

impl UserDeptDataScope {
    /// 每部门仅保留最高数据权限
    pub fn highest_scope_per_dept(
        dept_role_scope: Vec<UserDeptDataScope>,
    ) -> Vec<UserDeptDataScope> {
        // 构建一个以 部门ID 为键的 hashmap
        let mut dept_highest_scope = HashMap::with_capacity(dept_role_scope.len());

        // 遍历传入的 vec
        for entry in dept_role_scope.into_iter() {
            // 用闭包优化条件检查
            dept_highest_scope
                .entry(entry.dept_id)
                .and_modify(|existing: &mut UserDeptDataScope| {
                    // 如果部门下存在更高的数据权限，则替换
                    if entry.data_scope > existing.data_scope {
                        *existing = entry.clone();
                    }
                })
                .or_insert_with(|| entry);
        }

        // hashmap 还原为 vec
        dept_highest_scope.into_values().collect()
    }

    /// 判断是否存在 AllScope 所有数据权限
    pub fn contains_all_scope(dept_role_scope: &[UserDeptDataScope]) -> bool {
        dept_role_scope
            .iter()
            .any(|entry| entry.data_scope == DataScope::AllScope)
    }
}

// 获取或设置缓存
async fn get_or_set_cache<T, F>(
    redis_conn: &mut PooledConnection<'_, RedisConnectionManager>,
    key: &str,
    fetch_data: F,
) -> Result<T, Error>
where
    T: Serialize + for<'de> Deserialize<'de> + Send + Sync,
    F: Future<Output = Result<T, sqlx::Error>> + Send,
{
    // 检查缓存
    if let Some(cached_data) = redis_conn.get::<_, Option<Vec<u8>>>(key).await? {
        tracing::info!("Redis 权限缓存命中: {}", key);
        return Ok(serde_json::from_slice::<T>(&cached_data)?);
    }

    // 获取数据并缓存
    let data = fetch_data.await?;
    redis_conn
        .set::<_, _, ()>(key, serde_json::to_vec(&data)?)
        .await
        .map_err(|e| tracing::error!("Redis 权限缓存失败: {}", e))
        .ok();

    Ok(data)
}
