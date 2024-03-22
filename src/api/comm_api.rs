use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CurrentUser {
    pub id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JwtDnReq {
    #[serde(default)]
    pub name: Option<String>,
}
