use axum::extract::Query;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, Encode, MySql, MySqlPool, Row, Transaction};
use std::clone::Clone;

// 引入全局变量
use super::DB_POOL;
use crate::api::role_api;
#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Role {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub enable: i64,
}
impl Default for Role {
    fn default() -> Self {
        Role {
            id: 0,
            code: String::default(),
            name: String::default(),
            enable: 0,
        }
    }
}
// 查询多条记录
pub async fn fetch_all_where_user_id(uid: i64) -> Result<Vec<Role>, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let rows = sqlx::query_as::<_, Role>(
        "SELECT * FROM `role` WHERE id IN(SELECT roleId FROM user_roles_role WHERE userId=?)",
    )
    .bind(uid)
    .fetch_all(&pool)
    .await?;
    Ok(rows)
}
// 查询所有
pub async fn fetch_all_role() -> Result<Vec<Role>, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let rows = sqlx::query_as::<_, Role>("SELECT * FROM `role`")
        .fetch_all(&pool)
        .await?;
    Ok(rows)
}

// 查询所有
pub async fn fetch_all_by_req(req: Query<role_api::RolePageReq>) -> Result<Vec<Role>, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    // 构建 SQL 查询语句
    let mut sql_str = "SELECT * FROM `role`".to_string();
    let mut params: Vec<String> = Vec::new();
    if req.enable.is_some() || req.name.is_some() {
        sql_str.push_str(" WHERE");
        let mut conditions: Vec<String> = Vec::new();
        if let Some(enable) = req.enable {
            conditions.push(" enable=? ".to_string());
            params.push((&enable).to_string());
        }
        if let Some(name) = req.name.as_ref() {
            conditions.push(" `name` like ? ".to_string());
            params.push(format!("%{}%", name));
        }
        sql_str.push_str(&conditions.join(" AND"));
    }
    sql_str.push_str(" order by id desc LIMIT ? OFFSET ? ");
    let limit = req.pageSize.unwrap_or(10);
    let offset = (req.pageNo.unwrap_or(1) - 1) * 10;

    let query_builder = sqlx::query(&sql_str);
    let mut with_params = query_builder;
    for par in &params {
        with_params = with_params.bind(par);
    }
    with_params = with_params.bind(limit).bind(offset);

    let result = with_params.fetch_all(&pool).await?;
    let mut list: Vec<Role> = Vec::new();
    for row in result {
        let l = Role {
            // 从数据库行中提取信息并创建 Profile 对象
            id: row.get("id"),
            code: row.get("code"),
            name: row.get("name"),
            enable: row.get("enable"),
        };
        list.push(l);
    }

    Ok(list)
}

// 更新 enable 通过 id
pub async fn update_enable_by_id(enable: bool, id: i64) -> Result<MySqlQueryResult, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let result = sqlx::query("update role set enable = ? where id = ?")
        .bind(&enable)
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(result)
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}



// 更新 Role
pub async fn update_role_by_struct(
    pool: &mut Transaction<'_, MySql>,
    data: Role
) -> Result<bool, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let sql_str = "UPDATE role  SET code=?,name=?,enable=? where id =?  ";
    let result = sqlx::query(&sql_str)
        .bind(&data.code)
        .bind(&data.name)
        .bind(&data.enable)
        .bind(&data.id)
        .execute(&pool)
        .await?;
    let rows_aff = result.rows_affected();
    Ok(rows_aff > 0)
}