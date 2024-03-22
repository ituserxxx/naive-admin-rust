use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, MySql, Row, Transaction};
use std::clone::Clone;
// 引入全局变量
use super::DB_POOL;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub enable: i8,
    #[allow(non_snake_case)]
    pub createTime: DateTime<Utc>,
    #[allow(non_snake_case)]
    pub updateTime: DateTime<Utc>,
}
impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            username: String::default(),
            password: String::default(),
            enable: 0,
            createTime: Utc::now(),
            updateTime: Utc::now(),
        }
    }
}
// 查询一条记录-通过 username and password
pub async fn fetch_user_by_username_password(
    username: String,
    password: String,
) -> Result<Option<User>, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let result =
        sqlx::query_as::<_, User>("SELECT * FROM user where username = ? and password = ? ")
            .bind(&username)
            .bind(&password)
            .fetch_optional(&pool)
            .await?;
    Ok(result)
}

// 查询一条记录-通过 id
pub async fn find_info_by_id(id: i64) -> Result<Option<User>, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let result = sqlx::query_as::<_, User>("SELECT * FROM user where id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?;
    Ok(result)
}

// 更新记录-通过 id
pub async fn update_username_by_id(
    username: String,
    id: i64,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let result = sqlx::query("update user set username = ? where id = ?")
        .bind(&username)
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(result)
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}

// 更新 enable 通过 id
pub async fn update_enable_by_id(enable: bool, id: i64) -> Result<MySqlQueryResult, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let result = sqlx::query("update user set enable = ? where id = ?")
        .bind(&enable)
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(result)
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}

// 删除记录-通过 id (需要加事务，所以 pool 从外面传进来)
pub async fn delete_user_by_id(
    pool: &mut Transaction<'_, MySql>,
    id: i64,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("delete from user where id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
    let rows_affected = result.rows_affected();
    Ok(rows_affected == 1)
}

// 新增用户（需要加事务，所以 pool 从外面传进来）
pub async fn add_user_by_struct(
    pool: &mut Transaction<'_, MySql>,
    data: User,
) -> Result<u64, sqlx::Error> {
    let insert_sql = "INSERT INTO user (username, password, enable, createTime, updateTime) VALUES (?, ?, ?, ?, ?)";
    let result = sqlx::query(&insert_sql)
        .bind(&data.username)
        .bind(&data.password)
        .bind(&data.enable)
        .bind(&data.createTime)
        .bind(&data.updateTime)
        .execute(pool)
        .await?;

    // 获取新插入记录的 id
    let new_id = result.last_insert_id();
    Ok(new_id)
}

// 更新 password 通过 id
pub async fn update_password_by_id(
    password: String,
    id: i64,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let result = sqlx::query("update user set password = ? where id = ?")
        .bind(&password)
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(result)
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}
