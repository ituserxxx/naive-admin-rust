use crate::api::hello_api;
use crate::api::resp::ApiResponse;
use crate::tools::jwt;
use axum::extract::Json;

pub async fn jwt_en() {
    let token = jwt::en_token(21).await;
    println!("token: {:?}", token);
}

pub async fn jwt_dn(
    Json(req): Json<hello_api::JwtDnReq>,
) -> Json<ApiResponse<hello_api::JwtDnRes>> {
    let token = req.token.unwrap_or_default();
    match jwt::dn_token(token).await {
        Ok(uid) => {
            println!("Decoded UID: {}", uid);
            return Json(ApiResponse::succ(Some(hello_api::JwtDnRes {})));
        }
        Err(err) => {
            println!("Error: {}", err);
            return Json(ApiResponse::succ(Some(hello_api::JwtDnRes {})));
        }
    }
}

// 获取用户详情
// pub async fn detail2( Extension(curr_user): Extension<comm_api::CurrentUser>) -> Json<ApiResponse<user_api::UserInfoRes>> {
//     let id = curr_user.id;
//     let get_uinfo_result = user_model::fetch_user_by_id(id).await;
//     let uinfo = match get_uinfo_result {
//         Ok(Some(user)) =>user,
//         Ok(None) => {
//             return Json(ApiResponse::err( &"用户信息不存在"))
//         },
//         Err(err)=>{
//             let error_msg = format!("获取用户信息失败:{}", err);
//             return Json(ApiResponse::err( &error_msg))
//         }
//     };
//     // 初始化返回结构体
//     let rp = user_api::UserInfoRes {
//         info:uinfo,
//     };
//     return Json( ApiResponse::succ(Some(rp)))
// }

/*
use validator::Validate;
use chrono::Utc;
use axum::{
    middleware::{self, Next},
    extract::{Request, Extension,Json},
};

use crate::tools;
use crate::{
    db::{
        user_model,
        profile_model,
        role_model,
    },
    api::{
        user_api,
        comm_api,
    },
    api::resp::{
        ApiResponse
    },

};

// 获取用户详情
pub async fn detail( Extension(curr_user): Extension<comm_api::CurrentUser>) -> Json<ApiResponse<user_api::UserDetailRes>> {
    let uid = curr_user.id;
    let mut rp = user_api::UserDetailRes::default();

    // 通过uid获取 user信息
    let uinfo_result = user_model::find_info_by_id(uid).await;
    let uinfo = match uinfo_result {
        Ok(Some(a)) =>a,
        Ok(None) => {
            return Json(ApiResponse::err( &"用户信息不存在"))
        },
        Err(err)=>{
            let error_msg = format!("获取用户信息失败:{}", err);
            return Json(ApiResponse::err( &error_msg))
        }
    };

    rp.id = uinfo.id;
    rp.username = uinfo.username;
    rp.password = uinfo.password;
    rp.enable = uinfo.enable != 0;
    rp.createTime = uinfo.createTime.to_string();
    rp.updateTime = uinfo.updateTime.to_string();

    // 通过uid获取用户 Profile信息
    let pro_info_result = profile_model::find_info_by_user_id(uid).await;
    let pro_info = match pro_info_result {
        Ok(Some(a)) =>a,
        Ok(None) => {
            return Json(ApiResponse::err( &"profile信息不存在"))
        },
        Err(err)=>{
            let error_msg = format!("获取profile信息失败:{}", err);
            return Json(ApiResponse::err( &error_msg))
        }
    };
    rp.profile = pro_info;

    // 通过uid获取用户 role数组

    let roles_result = role_model::fetch_all_where_user_id(uid).await;
    let roles = match roles_result {
        Ok(rows) =>{
            if !rows.is_empty() {
                rows
            } else {
                Vec::new()
            }
        },
        Err(err)=>{
            let error_msg = format!("获取角色信息失败:{}", err);
            return Json(ApiResponse::err( &error_msg))
        }
    };
    rp.roles=roles.clone();
    if roles.len() >0 {
        rp.currentRole=roles[0].clone();
    }
    return Json( ApiResponse::succ(Some(rp)))
}


// 获取列表
pub async fn list() -> Json<ApiResponse<user_api::UserListRes>> {
    match  user_model::fetch_all_users().await {
        Ok(list) => {
            // 处理成功获取用户信息的情况
            return Json(ApiResponse::succ(Some(user_api::UserListRes {list:list})))
        }
        Err(err) => {
            // 处理查询失败的情况
            let error_msg = format!("err {}", err);
            return Json(ApiResponse::err(&error_msg))
        }
    }
}

// 新增用户
pub async fn add(Json(req): Json<user_api::AddUserReq>) -> Json<ApiResponse<user_api::AddUserResp>> {
    if let Err(error) = req.validate() {
        return Json( ApiResponse::new(400, None, &format!("{}", error)))
    }
    let username = req.username.unwrap_or_default();
    let password = req.password.unwrap_or_default();
    let new_time = Utc::now();
    let insert_user = user_model::User{
        id          :0,
        username    : username.to_string(),
        password    : tools::md5_crypto(password.to_string()),
        enable      :1,
        createTime  : new_time,
        updateTime  : new_time,
    };
    match user_model::add_user_by_struct(insert_user).await {
        Ok(insert_res) => {
            if insert_res.rows_affected() > 0 {
                // 初始化返回结构体
                let rp = user_api::AddUserResp {
                    id:insert_res.last_insert_id(),
                };
                return Json( ApiResponse::succ(Some(rp)))
            }
            return Json(ApiResponse::err( &"没有插入任何行"))
        }
        Err(err) => {
            let error_msg = format!("插入操作失败:{}", err);
            return Json(ApiResponse::err( &error_msg))
        }
    }
}

// 获取用户详情
pub async fn del(Json(req): Json<user_api::UserDelReq>) -> Json<ApiResponse<user_api::UserDelRes>> {
    if let Err(error) = req.validate() {
        return Json( ApiResponse::new(400, None, &format!("{}", error)))
    }
    let id = req.id.unwrap_or_default();

    let del_u_result = user_model::delete_user_by_id(id).await;

    match del_u_result {
        Ok(del_res) => {
            if del_res.rows_affected() > 0 {
                return Json( ApiResponse::succ(Some(user_api::UserDelRes{})))
            }
            return Json(ApiResponse::err( &"删除失败"))
        },
        Err(err)=>{
            let error_msg = format!("用户信息不存在:{}", err);
            return Json(ApiResponse::err( &error_msg))
        }
    };
}


 */
