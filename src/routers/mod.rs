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
        .route("/captcha", get(login::show_captcha))
        .route("/login", post(login::verify_captcha))
        .layer(SessionLayer::new(session_store));

    let user_router = Router::new()
        .route("/detail", get(user::detail))
        .route("/", post(user::add).get(user::list))
        .route("/:id", patch(user::patch_user).delete(user::del))
        .route("/profile/:id", patch(user::patch_profile))
        .route("/password/reset/:id", patch(user::reset_pwd))
        .layer(middleware::from_fn(auth::auth_jwt));

    let role_router = Router::new()
        .route("/", get(role::all))
        .route("/users/add/:id", patch(role::add_user))
        .route("/users/remove/:id", patch(role::remove_user))
        .route("/:id", patch(role::patch_role))
        .route("/permissions/tree", get(role::permissions_tree))
        .route("/page", get(role::page_list))
        .layer(middleware::from_fn(auth::auth_jwt));

    let permission_router = Router::new()
        .route("/tree", get(permission::tree))
        .layer(middleware::from_fn(auth::auth_jwt));

    return Router::new()
        .route("/", get(|| async { "☺ welcome to Rust" }))
        .nest("/hello", hello_router)
        .nest("/api/auth", auth_router)
        .nest(
            "/api/auth/password",
            Router::new()
                .route("/", post(user::update_passwd))
                .layer(middleware::from_fn(auth::auth_jwt)),
        )
        .nest("/api/user", user_router)
        .nest("/api/role", role_router)
        .nest("/api/permission", permission_router);
}
