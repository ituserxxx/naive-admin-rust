use axum::{
    extract::Extension,
    middleware::{self, Next},
    routing::{delete, get, patch, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use axum_session::{Session, SessionConfig, SessionLayer, SessionNullPool, SessionStore};
use std::sync::{Arc, Mutex};

use crate::{
    api::login_api, controllers::hello, controllers::login, controllers::permission,
    controllers::role, controllers::user, middleware::auth,
};

pub async fn init() -> Router {
    let hello_router = Router::new()
        .route("/jwt_en", get(hello::jwt_en))
        .route("/jwt_dn", post(hello::jwt_dn))
        .layer(middleware::from_fn(auth::auth_jwt));

    let session_config = SessionConfig::default().with_table_name("sessions_table");

    let session_store = SessionStore::<SessionNullPool>::new(None, session_config)
        .await
        .unwrap();

    let auth_router = Router::new()
        .route("/captcha", get(login::show_captcha)) // 获取（刷新）验证码
        .route("/login", post(login::verify_captcha)) // 用户登录
        .layer(SessionLayer::new(session_store));

    let user_router = Router::new()
        .route("/detail", get(user::detail)) //用户信息
        .route("/", post(user::add).get(user::list)) //用户新增 ----  用户列表
        .route("/:id", patch(user::patch_user).delete(user::del)) // 修改用户----删除用户
        .route("/profile/:id", patch(user::patch_profile)) //修改用户个人资料
        .route("/password/reset/:id", patch(user::reset_pwd)) //重置用户密码
        .layer(middleware::from_fn(auth::auth_jwt));

    let role_router = Router::new()
        .route("/", get(role::all)) //所有角色
        .route("/", post(role::add_role)) // 新增角色
        .route("/users/add/:id", patch(role::add_user)) // 角色分配（授权）用户
        .route("/users/remove/:id", patch(role::remove_user)) // 角色（批量）取消授权用户
        .route("/:id", patch(role::patch_role).delete(role::delete_role)) // 修改角色----删除角色
        .route("/permissions/tree", get(role::permissions_tree)) //角色菜单树
        .route("/page", get(role::page_list)) // 角色列表（筛选+分页）
        .layer(middleware::from_fn(auth::auth_jwt));

    let permission_router = Router::new()
        .route("/tree", get(permission::tree)) // 菜单树
        .layer(middleware::from_fn(auth::auth_jwt));

    return Router::new()
        .route("/", get(|| async { "☺ welcome to Rust" }))
        .nest("/hello", hello_router)
        .nest("/api/auth", auth_router)
        .nest(
            "/api/auth/password", // 用户修改密码
            Router::new()
                .route("/", post(user::update_passwd))
                .layer(middleware::from_fn(auth::auth_jwt)),
        )
        .nest("/api/user", user_router)
        .nest("/api/role", role_router)
        .nest("/api/permission", permission_router);
}
