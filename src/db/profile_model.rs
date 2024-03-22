use axum::extract::Query;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, Encode, MySql, Row, Transaction};
use std::clone::Clone;
// 引入全局变量
use super::DB_POOL;

use crate::api::user_api;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Profile {
    pub id: i64,
    pub gender: Option<i64>,
    pub avatar: String,
    pub address: Option<String>,
    pub email: Option<String>,
    #[allow(non_snake_case)]
    pub userId: i64,
    #[allow(non_snake_case)]
    pub nickName: Option<String>,
}
impl Default for Profile {
    fn default() -> Self {
        Profile {
            id: 0,
            gender: Some(0),
            avatar: String::default(),
            address: Some(String::default()),
            email: Some(String::default()),
            userId: 0,
            nickName: Some(String::default()),
        }
    }
}
pub async fn find_info_by_user_id(user_id: i64) -> Result<Option<Profile>, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let result = sqlx::query_as::<_, Profile>("SELECT * FROM profile where userId = ? ")
        .bind(user_id)
        .fetch_optional(&pool)
        .await?;
    Ok(result)
}

// 查询多条记录
pub async fn fetch_all_profile(
    req: Query<user_api::UserListReq>,
) -> Result<Vec<Profile>, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    // 构建 SQL 查询语句
    let mut sql_str = "SELECT * FROM profile".to_string();
    let mut params: Vec<String> = Vec::new();
    if req.enable.is_some() || req.gender.is_some() || req.username.is_some() {
        sql_str.push_str(" WHERE");
        let mut conditions: Vec<String> = Vec::new();
        if let Some(enable) = req.enable {
            conditions.push(" userId in(select id from user where enable=?) ".to_string());
            params.push((&enable).to_string());
        }
        if let Some(gender) = req.gender {
            conditions.push(" gender = ?".to_string());
            params.push((&gender).to_string());
        }
        if let Some(username) = req.username.as_ref() {
            conditions.push(" nickName like ? ".to_string());
            params.push(format!("%{}%", username));
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
    let mut list: Vec<Profile> = Vec::new();
    for row in result {
        let l = Profile {
            // 从数据库行中提取信息并创建 Profile 对象
            id: row.get("id"),
            gender: row.get("gender"),
            avatar: row.get("avatar"),
            address: row.get("address"),
            email: row.get("email"),
            userId: row.get("userId"),
            nickName: row.get("nickName"),
        };
        list.push(l);
    }

    Ok(list)
}

// 新增用户 Profile（需要加事务，所以 pool 从外面传进来）
pub async fn add_profile_by_struct(
    pool: &mut Transaction<'_, MySql>,
    data: Profile,
) -> Result<u64, sqlx::Error> {
    let insert_sql = "INSERT INTO profile (gender, avatar, address, email, userId,nickName ) VALUES (?, ?, ?, ?, ?,?)";
    let result = sqlx::query(&insert_sql)
        .bind(&data.gender)
        .bind(&data.avatar)
        .bind(&data.address)
        .bind(&data.email)
        .bind(&data.userId)
        .bind(&data.nickName)
        .execute(pool)
        .await?;
    // 获取新插入记录的 id
    let new_id = result.last_insert_id();
    Ok(new_id)
}

// 删除记录-通过 user_id (需要加事务，所以 pool 从外面传进来)
pub async fn delete_profile_by_user_id(
    pool: &mut Transaction<'_, MySql>,
    uid: i64,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("delete from profile where userId = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
    let rows_affected = result.rows_affected();
    Ok(rows_affected == 1)
}

// 更新用户 Profile
pub async fn update_profile_by_struct(data: Profile) -> Result<bool, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let sql_str = "UPDATE profile  SET gender=?, address=?, email=?, nickName=? where userId =?  ";
    let result = sqlx::query(&sql_str)
        .bind(&data.gender)
        .bind(&data.address)
        .bind(&data.email)
        .bind(&data.nickName)
        .bind(&data.userId)
        .execute(&pool)
        .await?;
    let rows_aff = result.rows_affected();
    Ok(rows_aff > 0)
}
// 更新用户 Profile avatar
pub async fn update_profile_avatar_by_user_id(
    avatar: String,
    userId: i64,
) -> Result<bool, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let sql_str = "UPDATE profile  SET avatar=?  where userId =?  ";
    let result = sqlx::query(&sql_str)
        .bind(avatar)
        .bind(userId)
        .execute(&pool)
        .await?;
    let rows_aff = result.rows_affected();
    Ok(rows_aff > 0)
}
