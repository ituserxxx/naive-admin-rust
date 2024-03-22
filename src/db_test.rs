// 测试数据库时 需要把此文件名改为 mian.rs 然后另外一个mian.rs 先重命名一下
mod tools;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions, MySqlQueryResult};
use std::clone::Clone;
use time::OffsetDateTime;

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, sqlx::FromRow)]
struct User {
    username: String,
    password: String,
    enable: i8,
    createTime: OffsetDateTime,
    updateTime: OffsetDateTime,
}

lazy_static! {
    static ref DB_POOL: Arc<Mutex<Option<sqlx::MySqlPool>>> = Arc::new(Mutex::new(None));
}
async fn init_pool() -> Result<sqlx::MySqlPool, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .connect("mysql://naive_admin:naive_admin_pass@localhost:33069/naive_admin")
        .await?;
    Ok(pool)
}

async fn shutdown_pool() -> Result<(), sqlx::Error> {
    if let Some(pool) = DB_POOL.lock().unwrap().take() {
        pool.close().await;
    }
    Ok(())
}
#[tokio::main]
async fn main() {
    // 初始化连接池
    let pool = init_pool().await.unwrap();

    // 存储连接池到全局变量
    *DB_POOL.lock().unwrap() = Some(pool);

    ope().await;

    // 释放连接池
    shutdown_pool().await.unwrap();
}
async fn ope() -> Result<(), sqlx::Error> {
    // 使用连接池进行数据库查询
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();

    //     let pool = MySqlPoolOptions::new().connect("mysql://naive_admin:naive_admin_pass@localhost:33069/naive_admin").await?;

    // let update_result = fetch_user_by_id(&pool,5).await;
    // match  update_result {
    //     Ok(user) => {
    //         // 处理成功获取用户信息的情况
    //         println!("1111 Successfully fetched user: {:?}", user);
    //         // 获取 name 字段的值
    //         let username = user.username;
    //         // 打印字段值
    //         println!("2222username: {:?}", username);
    //     }
    //     Err(err) => {
    //         // 处理查询失败的情况
    //         println!("Failed to fetch user: {:?}", err);
    //     }
    // }
    // Ok(())

    let user = User {
        username: "xxx".to_string(),
        password: tools::md5_crypto("123456".to_string()),
        enable: 1,
        createTime: OffsetDateTime::now_utc(),
        updateTime: OffsetDateTime::now_utc(),
    };
    let update_result = add_user_by_struct(&pool, user.clone()).await?;

    let users = fetch_all_users(&pool).await;
    println!("{:#?}", users);
    Ok(())

    //// ope MySqlTransaction ok
    //     let mut tx = pool.begin().await?;
    //
    //     let result = sqlx::query("delete from user where id = ?")
    //             .bind(6)
    //             .execute(&mut tx)
    //             .await;
    //     println!("delete result {:?}", result);
    //     // delete result Ok(MySqlQueryResult { rows_affected: 1, last_insert_id: 0 })
    //
    //     let result = sqlx::query("update user set username = ? where id = ?")
    //             .bind("John34".to_string())
    //             .bind(2)
    //             .execute(&mut tx).await;
    //     println!("update result {:?}", result);
    //     //// update result Ok(MySqlQueryResult { rows_affected: 1, last_insert_id: 0 })
    //     tx.commit().await?;
    // //     tx.rollback().await?;
    //
    // ////  ope  update ok
    //     let result = sqlx::query("update user set username = ? where id = ?")
    //                 .bind("John34".to_string())
    //                 .bind(2)
    //                 .execute(&pool).await;
    //         println!("update result {:?}", result);
    //// update result Ok(MySqlQueryResult { rows_affected: 1, last_insert_id: 0 })
    //
    // //// ope delete ok
    //     let result = sqlx::query("delete from user where id = ?").bind(6).execute(&pool).await;
    //     println!("delete result {:?}", result);
    //     // delete result Ok(MySqlQueryResult { rows_affected: 1, last_insert_id: 0 })

    ////   ope  insert ok
    //     let user = User {
    //         username: "John3".to_string(),
    //         password: tools::md5_crypto("123456".to_string()),
    //         enable:1,
    //         createTime: OffsetDateTime::now_utc(),
    //         updateTime: OffsetDateTime::now_utc(),
    //     };
    //     let insert_sql = "INSERT INTO user (username, password, enable, createTime, updateTime) VALUES (?, ?, ?, ?, ?)";
    //
    //     let result = sqlx::query(&insert_sql)
    //         .bind(&user.username)
    //         .bind(&user.password)
    //         .bind(&user.enable)
    //         .bind(&user.createTime)
    //         .bind(&user.updateTime)
    //         .execute(&pool)
    //         .await?;
    //
    //     println!("{:?}", result);
    //// MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }

    ////   ope  query ok
    //     let rows = sqlx::query_as::<_, User>("SELECT * FROM user").fetch_all(&pool).await?;
    //     println!("{:#?}", rows);
}

// fetch_user_by_id 调用示例
/*
    let update_result = fetch_user_by_id(&pool,5).await;
    match  update_result {
        Ok(user) => {
            // 处理成功获取用户信息的情况
            println!("Successfully fetched user: {:?}", user);
            // 获取 name 字段的值
            let username = user.username;
            // 打印字段值
            println!("username: {}", username);
        }
        Err(err) => {
            // 处理查询失败的情况
            eprintln!("Failed to fetch user: {}", err);
        }
    }
*/

async fn fetch_user_by_id(pool: &MySqlPool, id: i64) -> Result<User, sqlx::Error> {
    let result = sqlx::query_as::<_, User>("SELECT * FROM user where id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(result)
}

async fn fetch_all_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    let rows = sqlx::query_as::<_, User>("SELECT * FROM user")
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|row| row.clone()).collect())
}

async fn update_username_by_id(
    pool: &MySqlPool,
    username: &str,
    id: i64,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let result = sqlx::query("update user set username = ? where id = ?")
        .bind(username)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result)
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}
async fn delete_user_by_id(pool: &MySqlPool, id: i64) -> Result<(), sqlx::Error> {
    let result = sqlx::query("delete from user where id = ?")
        .bind(id)
        .execute(pool)
        .await;
    Ok(())
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}

async fn add_user_by_struct(pool: &MySqlPool, data: User) -> Result<MySqlQueryResult, sqlx::Error> {
    let insert_sql = "INSERT INTO user (username, password, enable, createTime, updateTime) VALUES (?, ?, ?, ?, ?)";
    let result = sqlx::query(&insert_sql)
        .bind(&data.username)
        .bind(&data.password)
        .bind(&data.enable)
        .bind(&data.createTime)
        .bind(&data.updateTime)
        .execute(pool)
        .await?;
    Ok(result)
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}
