mod scope;

use super::cache::{get_user_privileges, UserDeptDataScope};
use crate::{
    common::error::Error,
    state::{user::UserState, AppState},
};
use std::collections::HashSet;

// 导出数据范围
pub use scope::DataScope;

// 数据权限生成
pub async fn generate_user_limit(
    app_state: &AppState,
    user_state: &UserState,
    owner_column: &str,
    dept_column: &str,
) -> Result<String, Error> {
    // 管理员放行
    if user_state.is_admin() {
        return Ok("1 = 1".to_string());
    }

    // 获取用户所有权限
    let user_priv = get_user_privileges(app_state, user_state.get_id()).await?;

    // 如果用户包含所有数据权限，则无需再做限制
    if UserDeptDataScope::contains_all_scope(&user_priv) {
        return Ok("1 = 1".to_string());
    }

    // 获取用户对各部门的最高数据权限
    let user_priv = UserDeptDataScope::highest_scope_per_dept(user_priv);

    // 初始化条件，必有本人且仅有一次
    let mut conditions = vec![format!("{} = {}", owner_column, user_state.get_id())];

    // 遍历权限，汇总部门ID
    let mut dept_ids = HashSet::new();
    for entry in user_priv.iter() {
        match entry.data_scope {
            // 汇总部门ID
            DataScope::DeptScope | DataScope::HircScope => {
                dept_ids.insert(entry.dept_id);
            }
            // 其他已处理，跳过
            _ => continue,
        }
    }

    // 处理部门ID汇总
    if !dept_ids.is_empty() {
        let dept_ids_str = dept_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        conditions.push(format!("{dept_column} IN ({dept_ids_str})"));
    }

    // 拼接最终查询条件
    Ok(conditions.join(" OR "))
}
