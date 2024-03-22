use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PermissionItem {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub r#type: String,
    #[allow(non_snake_case)]
    pub parentId: Option<i64>,
    pub path: Option<String>,
    pub redirect: Option<String>,
    pub icon: Option<String>,
    pub component: Option<String>,
    pub layout: Option<String>,
    #[allow(non_snake_case)]
    pub keepAlive: Option<i8>,
    pub method: Option<String>,
    pub description: Option<String>,
    pub show: i8,
    pub enable: i8,
    pub order: i64,
    pub children: Option<Vec<Box<PermissionItem>>>,
}

#[derive(Debug, Clone, Validate, Default, Deserialize, Serialize)]
pub struct RolePageReq {
    #[allow(non_snake_case)]
    pub pageNo: Option<i64>, //  可传：默认1
    #[allow(non_snake_case)]
    pub pageSize: Option<i64>, //  可传：默认10
    pub name: Option<String>,
    pub enable: Option<i64>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RolePageResp {
    #[allow(non_snake_case)]
    pub pageData: Option<Vec<RolePageItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RolePageItem {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub enable: bool,
    #[allow(non_snake_case)]
    pub permissionIds: Option<Vec<i64>>,
}
#[derive(Debug, Clone, Validate, Default, Deserialize, Serialize)]
pub struct RolePatchReq {
    pub enable: bool,
    pub name: Option<String>, // 根据这个判断是否是修改状态
    pub code: Option<String>,
    #[allow(non_snake_case)]
    pub permissionIds: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Validate, Default, Deserialize, Serialize)]
pub struct RoleAddUserReq {
    #[allow(non_snake_case)]
    pub userIds: Vec<i64>,
}
