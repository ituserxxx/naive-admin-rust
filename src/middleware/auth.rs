use axum::{
    extract::{Extension, Request},
    http,
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
};

use axum_extra::extract::WithRejection;
use thiserror::Error;

use crate::api::comm_api;
use crate::tools::jwt;

pub async fn auth_jwt(req: Request, next: Next) -> Response {
    match handle_auth_jwt(req, next).await {
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

async fn handle_auth_jwt(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());
    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    match jwt::dn_token(auth_header.to_string()).await {
        Ok(uid) => {
            req.extensions_mut()
                .insert(comm_api::CurrentUser { id: uid });
            Ok(next.run(req).await)
        }
        Err(err) => {
            // 处理解码失败的情况
            return Err(StatusCode::UNAUTHORIZED);
        }
    }
}
