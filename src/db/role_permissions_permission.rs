use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, Encode, MySql, MySqlPool, Row, Transaction};
use std::clone::Clone;
// 引入全局变量
use super::DB_POOL;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct RolePermissionsPermission {
    #[allow(non_snake_case)]
    pub roleId: i64,
    #[allow(non_snake_case)]
    pub permissionId: i64,
}
impl Default for RolePermissionsPermission {
    fn default() -> Self {
        RolePermissionsPermission {
            roleId: 0,
            permissionId: 0,
        }
    }
}

// 查询一个字段记录，返回数组值
pub async fn fetch_permission_ids_where_role_id(mut role_id: i64) -> Result<Vec<i64>, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    // 注意：这里不能查单个字段，因为下面用了query_as 映射结构体，这样会报错； ColumnNotFound("roleId")
    // let mut sql_str = String::from("SELECT permissionId FROM role_permissions_permission ");

    let mut sql_str = String::from("SELECT * FROM role_permissions_permission ");
    // 如果不是超级管理员，则添加 WHERE 子句
    if role_id != 1 {
        sql_str.push_str("WHERE roleId = ?");
    }
    if role_id == 1 {
        role_id = 0;
        // 这里只是兼容了一下，因为下面必须 bind（role_id）
        sql_str.push_str("WHERE roleId > ?");
    }
    let rows: Vec<RolePermissionsPermission> = sqlx::query_as(&sql_str)
        .bind(role_id)
        .fetch_all(&pool)
        .await?;
    // 提取 permissionId 列的值并转换为 i64 数组
    let permission_ids: Vec<i64> = rows.iter().map(|row| row.permissionId).collect();
    Ok(permission_ids)
}
// 删除角色权限关系记录-通过 role_id (需要加事务，所以 pool 从外面传进来)
pub async fn delete_permissions_by_role_id(
    pool: &mut Transaction<'_, MySql>,
    role_id: i64,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query("delete from role_permissions_permission where roleId = ?")
        .bind(role_id)
        .execute(pool)
        .await?;
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
    let rows_affected = result.rows_affected();
    Ok(())
}

// 新增角色权限关系（需要加事务，所以 pool 从外面传进来）
pub async fn add_role_permissions_by_struct(
    pool: &mut Transaction<'_, MySql>,
    data: RolePermissionsPermission,
) -> Result<u64, sqlx::Error> {
    let insert_sql = "INSERT INTO role_permissions_permission (permissionId, roleId) VALUES (?, ?)";
    let result = sqlx::query(&insert_sql)
        .bind(&data.permissionId)
        .bind(&data.roleId)
        .execute(pool)
        .await?;
    // 获取新插入记录的 id
    let new_id = result.last_insert_id();
    Ok(new_id)
}