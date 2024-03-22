use axum::{
    body::Body,
    extract::rejection::JsonRejection,
    extract::{Extension, Request},
    http,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};

use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CurrentUser {
    pub id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JwtDnReq {
    #[serde(default)]
    pub name: Option<String>,
}

async fn auth_demo(req: Request, next: Next) -> Response {
    match handle_auth_demo(req, next).await {
        Ok(response) => response,
        Err(status_code) => {
            let body = format!("Error: {}", status_code);
            Response::builder()
                .status(status_code)
                .body(body.into())
                .unwrap()
        }
    }
}

async fn handle_auth_demo(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    if let Some(current_user) = authorize_current_user(auth_header).await {
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }
}

async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    return Some(CurrentUser { id: 1 });
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UserInfoReq {
    #[serde(default)]
    #[validate(required)]
    pub id: Option<i64>,
}

pub async fn handler(
    Extension(current_user): Extension<CurrentUser>,
    Json(req): Json<UserInfoReq>,
) -> Json<Option<CurrentUser>> {
    if let Err(error) = req.validate() {
        return Json(Some(CurrentUser { id: -1 }));
    }
    println!("req: {:?}", req);
    println!("Current user: {:?}", current_user);
    return Json(Some(CurrentUser { id: 123 }));
}
pub async fn handler2(Json(req): Json<UserInfoReq>) -> Json<Option<CurrentUser>> {
    if let Err(error) = req.validate() {
        return Json(Some(CurrentUser { id: -1 }));
    }
    println!("req: {:?}", req);
    //       println!("Current user: {:?}", current_user);
    return Json(Some(CurrentUser { id: 123 }));
}

#[tokio::main]
async fn main() {
    // curl -X GET -H "Content-Type: application/json" -H "Authorization: xxxxxxxxx" -d '{"id":1111}' http://127.0.0.1:8061/1
    // curl -X GET -H "Content-Type: application/json" -H "Authorization: xxxxxxxxx" -d '{"id":1111}' http://127.0.0.1:8061/2
    // curl -X GET -H "Content-Type: application/json" -H "Authorization: xxxxxxxxx" -d '{"id":1111}' http://127.0.0.1:8061/3
    // curl -X GET -H "Content-Type: application/json"  -d '{"name":"xx1"}' http://127.0.0.1:8061/

    let app = Router::new()
        .route("/1", get(handler))
        .route("/3", get(handler))
        .layer(middleware::from_fn(auth_demo))
        .route("/2", get(handler2));
    let router_init = app.into_make_service();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8061").await.unwrap();
    println!("server port on {}", "0.0.0.0:8061");
    // 启动服务
    axum::serve(listener, router_init).await.unwrap();
}
