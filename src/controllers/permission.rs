use axum::{
    extract::{Extension, Json, Request},
    middleware::{self, Next},
};
use chrono::Utc;
use std::rc::Rc;
use validator::Validate;

use crate::tools;
use crate::{
    api::resp::ApiResponse,
    api::{comm_api, role_api::PermissionItem, user_api},
    db::{permission_model, profile_model, role_model, user_model, user_roles_role_model},
};

pub async fn tree(
    Extension(curr_user): Extension<comm_api::CurrentUser>,
) -> Json<ApiResponse<Option<Vec<PermissionItem>>>> {
    let mut one_arr: Vec<permission_model::Permission> = Vec::new();
    let find_1_level_result = permission_model::find_1_level().await;
    one_arr = match find_1_level_result {
        Ok(a) => a,
        Err(err) => {
            let error_msg = format!("获取所有权限信息失败:{:?}", err);
            return Json(ApiResponse::err(&error_msg));
        }
    };
    let mut rp_arr: Vec<PermissionItem> = Vec::new();
    for one in one_arr {
        let mut m1 = Box::new(PermissionItem {
            id: one.id,
            name: one.name,
            code: one.code,
            r#type: one.r#type,
            parentId: one.parentId,
            path: one.path,
            redirect: one.redirect,
            icon: one.icon,
            component: one.component,
            layout: one.layout,
            keepAlive: one.keepAlive,
            method: one.method,
            description: one.description,
            show: one.show,
            enable: one.enable,
            order: one.order,
            children: Some(Vec::new()),
        });
        let find_2_result = permission_model::find_all_where_by_p_id(one.id).await;
        if let Ok(two_arr) = find_2_result {
            let mut two_children: Vec<PermissionItem> = Vec::new();
            for two in two_arr {
                let mut m2 = PermissionItem {
                    id: two.id,
                    name: two.name,
                    code: two.code,
                    r#type: two.r#type,
                    parentId: two.parentId,
                    path: two.path,
                    redirect: two.redirect,
                    icon: two.icon,
                    component: two.component,
                    layout: two.layout,
                    keepAlive: two.keepAlive,
                    method: two.method,
                    description: two.description,
                    show: two.show,
                    enable: two.enable,
                    order: two.order,
                    children: Some(Vec::new()),
                };
                let find_3_result = permission_model::find_all_where_by_p_id(two.id).await;
                if let Ok(three_arr) = find_3_result {
                    let mut three_children: Vec<PermissionItem> = Vec::new();
                    for three in three_arr {
                        let m3 = PermissionItem {
                            id: three.id,
                            name: three.name,
                            code: three.code,
                            r#type: three.r#type,
                            parentId: three.parentId,
                            path: three.path,
                            redirect: three.redirect,
                            icon: three.icon,
                            component: three.component,
                            layout: three.layout,
                            keepAlive: three.keepAlive,
                            method: three.method,
                            description: three.description,
                            show: three.show,
                            enable: three.enable,
                            order: three.order,
                            children: Some(Vec::new()),
                        };
                        three_children.push(m3)
                    }
                    m2.children = Some(three_children.into_iter().map(Box::new).collect());
                }
                two_children.push(m2)
            }
            // 将二级权限列表赋值给一级权限的子节点
            m1.children = Some(two_children.into_iter().map(Box::new).collect());
        }
        rp_arr.push(*m1);
    }
    return Json(ApiResponse::succ(Some(Some(rp_arr))));
}
