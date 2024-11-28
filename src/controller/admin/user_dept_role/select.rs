use super::UserPrivilege;
use crate::{
    common::error::Error,
    model::sys::{
        dept::TABLE_SYS_DEPT,
        role::TABLE_SYS_ROLE,
        user_dept_role::{UserDeptRoleWithName, TABLE_SYS_USER_DEPT_ROLE},
    },
};
use sqlx::mysql::MySqlPool;
use std::collections::HashMap;

// 获取用户权限
pub async fn query_users_dept_role(
    db: &MySqlPool,
    user_ids: Vec<i64>,
) -> Result<HashMap<i64, Vec<UserPrivilege>>, Error> {
    if user_ids.len() <= 0 {
        return Ok(HashMap::new());
    }

    // 数组转字符串
    let ids = user_ids
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(", ");

    // 查询
    let query = format!(
        r#"SELECT udr.user_id, udr.dept_id, udr.role_id, 
        d.name AS dept_name, r.name AS role_name 
        FROM `{TABLE_SYS_USER_DEPT_ROLE}` udr 
        LEFT JOIN `{TABLE_SYS_DEPT}` d ON d.id = udr.dept_id 
        LEFT JOIN `{TABLE_SYS_ROLE}` r ON r.id = udr.role_id 
        WHERE udr.user_id IN ({ids})"#
    );
    let udrs: Vec<UserDeptRoleWithName> = sqlx::query_as(&query).fetch_all(db).await?;

    // 转换数据为HashMap
    let mut result: HashMap<i64, Vec<UserPrivilege>> = HashMap::new();
    udrs.into_iter().for_each(|udr| {
        result.entry(udr.user_id).or_default().push(UserPrivilege {
            dept_id: Some(udr.dept_id),
            dept_name: udr.dept_name,
            role_id: Some(udr.role_id),
            role_name: udr.role_name,
        });
    });

    Ok(result)
}
