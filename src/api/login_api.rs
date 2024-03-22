use serde::{Deserialize, Serialize};
use validator::Validate;
// 会话数据结构体
#[derive(Debug, Default)]
pub struct SessionData {
    pub captcha: Option<String>,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct LoginReq {
    #[serde(default)]
    pub captcha: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResp {
    #[allow(non_snake_case)]
    pub accessToken: String,
}
