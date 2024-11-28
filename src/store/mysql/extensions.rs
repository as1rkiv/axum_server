#![allow(dead_code)]

use crate::{
    common::error::Error,
    middleware::privilege::data::generate_user_limit,
    state::{user::UserState, AppState},
};
use sqlx::{Database, Encode, MySql, QueryBuilder, Type};

/// Sqlx 查询拓展
pub trait QueryBuilderExt<'args, DB>
where
    DB: Database,
{
    /// 字段查询
    fn select(select: &str, from: &str, alias: &str) -> Self;

    /// WHERE 1 = 1
    fn maybe(&mut self) -> &mut Self;

    /// OR
    fn or(&mut self) -> &mut Self;

    /// AND
    fn and(&mut self) -> &mut Self;

    /// 链接查询
    fn join(&mut self, join_on: String) -> &mut Self;

    /// 等于
    fn eq<T>(&mut self, field: &str, eq: T) -> &mut Self
    where
        T: 'args + Encode<'args, DB> + Type<DB>;

    /// 不等于
    fn not_eq<T>(&mut self, field: &str, not_eq: T) -> &mut Self
    where
        T: 'args + Encode<'args, DB> + Type<DB>;

    /// 相似
    fn like<'a>(&mut self, field: &str, like: &'a str) -> &mut Self
    where
        'a: 'args;

    /// 空值
    fn is_null(&mut self, field: &str) -> &mut Self;

    /// 非空值
    fn is_not_null(&mut self, field: &str) -> &mut Self;

    /// 软删除
    fn is_deleted(&mut self, table: &str) -> &mut Self;

    /// 非软删除
    fn is_not_deleted(&mut self, table: &str) -> &mut Self;

    /// 排序
    fn order_by(&mut self, by: &str) -> &mut Self;

    /// 分组
    fn group_by(&mut self, by: &str) -> &mut Self;

    /// 限制数量
    fn limit(&mut self, size: i64) -> &mut Self;

    /// 分页
    fn pagination(&mut self, page_index: i64, page_size: i64) -> &mut Self;

    /// 数据权限
    async fn privileges(
        &mut self,
        app_state: &AppState,
        user_state: &UserState,
        owner_column: &str,
        dept_column: &str,
    ) -> Result<&mut Self, Error>;
}

/// MySql 查询拓展
impl<'args> QueryBuilderExt<'args, MySql> for QueryBuilder<'args, MySql> {
    /// 字段查询
    fn select(select: &str, from: &str, alias: &str) -> Self {
        QueryBuilder::<MySql>::new(format!(r#"SELECT {select} FROM `{from}` {alias}"#))
    }

    /// WHERE 1 = 1
    fn maybe(&mut self) -> &mut Self {
        self.push(r#" WHERE 1 = 1"#);
        self
    }

    /// OR
    fn or(&mut self) -> &mut Self {
        self.push(r#" OR"#);
        self
    }

    /// AND
    fn and(&mut self) -> &mut Self {
        self.push(r#" AND"#);
        self
    }

    /// 链接查询
    fn join(&mut self, join_on: String) -> &mut Self {
        self.push(r#" "#).push(join_on);
        self
    }

    /// 等于
    fn eq<T>(&mut self, field: &str, eq: T) -> &mut Self
    where
        T: 'args + Encode<'args, MySql> + Type<MySql>,
    {
        self.push(format!(r#" {field} = "#)).push_bind(eq);
        self
    }

    /// 不等于
    fn not_eq<T>(&mut self, field: &str, not_eq: T) -> &mut Self
    where
        T: 'args + Encode<'args, MySql> + Type<MySql>,
    {
        self.push(format!(r#" {field} <> "#)).push_bind(not_eq);
        self
    }

    /// 相似
    fn like<'a>(&mut self, field: &str, like: &'a str) -> &mut Self
    where
        'a: 'args,
    {
        self.push(format!(r#" {field} LIKE '%{like}%'"#));
        self
    }

    /// 空值
    fn is_null(&mut self, field: &str) -> &mut Self {
        self.push(format!(r#" {field} IS NULL"#));
        self
    }

    /// 非空值
    fn is_not_null(&mut self, field: &str) -> &mut Self {
        self.push(format!(r#" {field} IS NOT NULL"#));
        self
    }

    /// 软删除
    fn is_deleted(&mut self, table: &str) -> &mut Self {
        self.push(format!(r#" {table}.is_deleted = 1"#));
        self
    }

    /// 非软删除
    fn is_not_deleted(&mut self, table: &str) -> &mut Self {
        self.push(format!(r#" {table}.is_deleted = 0"#));
        self
    }

    /// 排序
    fn order_by(&mut self, by: &str) -> &mut Self {
        self.push(format!(r#" ORDER BY {by}"#));
        self
    }

    /// 分组
    fn group_by(&mut self, by: &str) -> &mut Self {
        self.push(format!(r#" GROUP BY {by}"#));
        self
    }

    /// 限制数量
    fn limit(&mut self, size: i64) -> &mut Self {
        self.push(r#" LIMIT "#).push_bind(size);
        self
    }

    /// 分页
    fn pagination(&mut self, page_index: i64, page_size: i64) -> &mut Self {
        self.push(r#" LIMIT "#)
            .push_bind(page_size)
            .push(r#" OFFSET "#)
            .push_bind((page_index - 1) * page_size);
        self
    }

    /// 数据权限
    async fn privileges(
        &mut self,
        app_state: &AppState,
        user_state: &UserState,
        owner_column: &str,
        dept_column: &str,
    ) -> Result<&mut Self, Error> {
        self.push(r#" "#)
            .push(generate_user_limit(app_state, user_state, owner_column, dept_column).await?);
        Ok(self)
    }
}
