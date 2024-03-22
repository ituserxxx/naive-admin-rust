use axum::{
    extract, extract::Json, extract::Query, extract::Request, routing::get, routing::post, Router,
};
use axum_session::{Session, SessionConfig, SessionLayer, SessionNullPool, SessionStore};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
#[derive(Deserialize)]
struct GreetQuery {
    name: String,
}
#[tokio::main]
async fn main() {
    let session_config = SessionConfig::default().with_table_name("sessions_table");

    // create SessionStore and initiate the database tables
    let session_store = SessionStore::<SessionNullPool>::new(None, session_config)
        .await
        .unwrap();

    // build our application with some routes
    let app = Router::new()
        .route("/greet", get(greet))
        .route("/greet2", get(greet))
        .route("/greet3", post(greet3))
        .route("/greet4", post(greet3))
        .layer(SessionLayer::new(session_store));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    println!("server port on {}", "0.0.0.0:8800");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
#[derive(Deserialize, Debug)]
struct CreateUser {
    email: String,
    password: String,
}
async fn greet(query: Query<GreetQuery>, session: Session<SessionNullPool>) -> String {
    let name = &query.name;
    println!("Hello, {}!", name);

    let mut count: usize = session.get("count").unwrap_or(0);
    println!("count {}", count);
    count += 1;
    session.set("count", count);
    count.to_string()
}

use axum::Form;

#[derive(Deserialize, Debug)]
struct SignUp {
    username: String,
    password: String,
}
async fn greet3(session: Session<SessionNullPool>, Form(sign_up): Form<SignUp>) -> String {
    println!("Hello, {:?}", sign_up.username);
    let mut count: usize = session.get("count").unwrap_or(0);
    println!("count {}", count);
    count += 1;
    session.set("count", count);
    count.to_string()
}

async fn greet4(session: Session<SessionNullPool>, Json(req): Json<SignUp>) -> String {
    println!("Hello, {:?}", req);
    let mut count: usize = session.get("count").unwrap_or(0);
    println!("count {}", count);
    count += 1;
    session.set("count", count);
    count.to_string()
}
