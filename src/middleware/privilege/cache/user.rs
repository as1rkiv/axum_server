use super::{get_or_set_cache, DataScope, UserDeptDataScope, KEY_DEPT_DATASCOPE};
use crate::{
    common::error::Error,
    model::sys::{
        dept::TABLE_SYS_DEPT, role::TABLE_SYS_ROLE, user_dept_role::TABLE_SYS_USER_DEPT_ROLE,
    },
    state::AppState,
};
use redis::AsyncCommands;

/// 获取某人的角色继承关系
/// 如果redis中存在，则使用缓存
/// 如果不存在，则查询数据库
/// 将结果写入缓存后再返回
pub async fn get_user_privileges(
    app_state: &AppState,
    user_id: i64,
) -> Result<Vec<UserDeptDataScope>, Error> {
    // 获取redis连接池
    let mut redis_conn = app_state.redis_privilege().get().await?;

    // 拼接redis键
    let redis_key = format!("{KEY_DEPT_DATASCOPE}:{user_id}");

    // 递归查询用户部门与角色数据范围
    // 当用户在某部门角色权限为 HircScope 时，才会递归继续向下查询
    // 此处获得的结果，是用户拥有权限的所有部门ID
    // 数据权限处理时，只需要判断数据的部门ID是否在此集合中即可
    let query_user_privileges = format!(
        r#"WITH RECURSIVE Privileges AS (
            SELECT udr.dept_id, r.data_scope 
            FROM `{TABLE_SYS_USER_DEPT_ROLE}` udr 
            INNER JOIN `{TABLE_SYS_ROLE}` r 
            ON r.is_deleted = 0 AND r.id = udr.role_id 
            WHERE udr.user_id = ? 

            UNION ALL 

            SELECT dept.id AS dept_id, p.data_scope 
            FROM Privileges p 
            INNER JOIN `{TABLE_SYS_DEPT}` dept 
            ON dept.is_deleted = 0 AND p.data_scope = {scope} 
            AND dept.pid = p.dept_id 
        ) SELECT dept_id, data_scope FROM Privileges;"#,
        scope = DataScope::HircScope
    );

    // Redis 中没有缓存，查询数据库
    let query = sqlx::query_as(&query_user_privileges)
        .bind(user_id)
        .fetch_all(app_state.mysql());

    get_or_set_cache(&mut redis_conn, &redis_key, query).await
}

/// 删除某个用户的鉴权缓存
pub async fn del_user_privileges_by_id(app_state: &AppState, user_id: i64) -> Result<(), Error> {
    // 获取redis连接池
    let mut redis_conn = app_state.redis_privilege().get().await?;

    // 拼接redis键
    let redis_key = format!("{KEY_DEPT_DATASCOPE}:{user_id}");

    // 删除键
    redis_conn
        .del::<_, ()>(redis_key)
        .await
        .map_err(|e| tracing::error!("Redis 错误: {}", e))
        .ok();

    Ok(())
}

/// 删除全部用户的鉴权缓存
pub async fn del_users_privileges(app_state: &AppState) -> Result<(), Error> {
    // 获取redis连接池
    let mut redis_conn = app_state.redis_privilege().get().await?;

    // 清空鉴权库
    redis_conn
        .send_packed_command(&redis::cmd("FLUSHDB"))
        .await
        .map_err(|e| tracing::error!("Redis 错误: {}", e))
        .ok();

    Ok(())
}
