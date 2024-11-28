use super::Recursion;
use crate::common::error::Error;
use sqlx::{mysql::MySql, Acquire, Transaction};

impl Recursion {
    /// 查询 MySql 继承关系链冲突
    pub async fn has_circular_dependency(
        tx: &mut Transaction<'_, MySql>,
        table_name: &str,
        id: i64,
        pid: i64,
    ) -> Result<bool, Error> {
        // 获取事务所在的连接
        let conn = tx.acquire().await?;

        // 锁定相关行且不等待，减少死锁风险
        let lock_row =
            format!(r#"SELECT id FROM `{table_name}` WHERE id IN (?, ?) FOR UPDATE NOWAIT"#);
        sqlx::query(&lock_row)
            .bind(pid)
            .bind(id)
            .execute(&mut *conn)
            .await?;

        // 检查新 pid 是否在 id 的上级链中
        let upward_query = format!(
            r#"WITH RECURSIVE ancestors AS ( 
          SELECT id, pid FROM `{table_name}` WHERE id = ? 
          UNION ALL 
          SELECT d.id, d.pid FROM `{table_name}` d 
          INNER JOIN ancestors a ON d.id = a.pid 
        ) SELECT EXISTS (SELECT 1 FROM ancestors WHERE id = ?);"#
        );

        // 如果上级链检测到循环依赖，立即返回，避免执行下级链查询
        if sqlx::query_scalar(&upward_query)
            .bind(pid)
            .bind(id)
            .fetch_one(&mut *conn)
            .await?
        {
            return Ok(true);
        }

        // 检查 id 是否在新 pid 的下级链中
        let downward_query = format!(
            r#"WITH RECURSIVE descendants AS ( 
          SELECT id, pid FROM `{table_name}` WHERE pid = ? 
          UNION ALL 
          SELECT d.id, d.pid FROM `{table_name}` d 
          INNER JOIN descendants a ON d.pid = a.id 
        ) SELECT EXISTS (SELECT 1 FROM descendants WHERE id = ?);"#
        );

        // 下级链查询
        Ok(sqlx::query_scalar(&downward_query)
            .bind(id)
            .bind(pid)
            .fetch_one(&mut *conn)
            .await?)
    }
}
