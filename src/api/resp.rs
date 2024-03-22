use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    code: u32,
    //     #[serde(default)]
    data: Option<T>,
    message: String,
    // originUrl: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn new(code: u32, data: Option<T>, msg: &str) -> Self {
        ApiResponse {
            code,
            data,
            message: msg.to_string(),
        }
    }
    pub fn succ(data: Option<T>) -> Self {
        ApiResponse {
            code: 0,
            data,
            message: "ok".to_string(),
        }
    }
    pub fn err(msg: &str) -> Self {
        ApiResponse {
            code: 500,
            data: None,
            message: msg.to_string(),
        }
    }
}
