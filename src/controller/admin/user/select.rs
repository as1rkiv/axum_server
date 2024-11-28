use super::{super::user_dept_role::select, UserListResp};
use crate::{
    common::{error::Error, request::Pagination},
    model::sys::{
        user::{User, TABLE_SYS_USER},
        user_dept_role::TABLE_SYS_USER_DEPT_ROLE,
    },
    store::mysql::QueryBuilderExt,
};
use serde::Deserialize;
use sqlx::{MySqlPool, QueryBuilder};
use validator::Validate;

// 用户列表查询项
#[derive(Debug, Deserialize, Validate)]
pub struct UserListQuery {
    pub username: Option<String>,
    pub fullname: Option<String>,
    pub role: Option<String>,
    pub dept: Option<String>,
}

// 获取用户列表
pub async fn query_user_list(
    db: &MySqlPool,
    params: Pagination<UserListQuery>,
) -> Result<(i64, Vec<UserListResp>), Error> {
    // 查询数量
    let mut count_builder =
        QueryBuilder::select("COUNT(DISTINCT u.id) AS count", TABLE_SYS_USER, "u");
    count_builder.join(format!(
        "LEFT JOIN `{TABLE_SYS_USER_DEPT_ROLE}` udr ON udr.user_id = u.id"
    ));

    // 查询字段
    let mut query_builder = QueryBuilder::select("DISTINCT u.*", TABLE_SYS_USER, "u");
    query_builder.join(format!(
        "LEFT JOIN `{TABLE_SYS_USER_DEPT_ROLE}` udr ON udr.user_id = u.id"
    ));

    // 排除管理员
    count_builder
        .maybe()
        .and()
        .is_not_deleted("u")
        .and()
        .not_eq("u.id", 1);
    query_builder
        .maybe()
        .and()
        .is_not_deleted("u")
        .and()
        .not_eq("u.id", 1);

    // 姓名过滤
    if let Some(fullname) = &params.get_params().fullname {
        if !fullname.trim().is_empty() {
            let trimmed_fullname = fullname.trim();
            count_builder.and().like("u.fullname", trimmed_fullname);
            query_builder.and().like("u.fullname", trimmed_fullname);
        }
    }

    // 账号过滤
    if let Some(username) = &params.get_params().username {
        if !username.trim().is_empty() {
            let trimmed_username = username.trim();
            count_builder.and().like("u.username", trimmed_username);
            query_builder.and().like("u.username", trimmed_username);
        }
    }

    // 角色
    if let Some(role_id) = &params.get_params().role {
        if !role_id.is_empty() && role_id.parse::<i64>().is_ok() {
            count_builder.and().eq("udr.role_id", role_id);
            query_builder.and().eq("udr.role_id", role_id);
        }
    }

    // 部门
    if let Some(dept_id) = &params.get_params().dept {
        if !dept_id.is_empty() && dept_id.parse::<i64>().is_ok() {
            count_builder.and().eq("udr.dept_id", dept_id);
            query_builder.and().eq("udr.dept_id", dept_id);
        }
    }

    // 查询数量
    let count: i64 = match count_builder.build_query_scalar().fetch_one(db).await? {
        0 => return Ok((0, vec![])), // 数量为0时直接返回空列表
        count => count,
    };

    // 获取所有用户
    let users: Vec<User> = query_builder
        .order_by("u.id DESC")
        .pagination(params.get_index(), params.get_size())
        .build_query_as()
        .fetch_all(db)
        .await?;

    // 查询用户权限
    let user_ids: Vec<i64> = users.iter().map(|u| u.id).collect();
    let udrs = select::query_users_dept_role(db, user_ids).await?;

    // 组合结果
    let list: Vec<UserListResp> = users
        .into_iter()
        .map(|user| UserListResp {
            privileges: udrs.get(&user.id).cloned().unwrap_or_default(),
            user,
        })
        .collect();

    Ok((count, list))
}
