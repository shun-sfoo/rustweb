use std::{env, net::SocketAddr};

use axum::{
    routing::{get, post},
    Extension, Router,
};
use sea_orm::Database;
use service::{me, register};

mod db;
mod model;
mod service;

#[tokio::main]
async fn main() {
    // 设置日志，测试等级为debug
    env::set_var("RUST_LOG", "debug");
    // 获取环境参数
    dotenv::dotenv().ok();
    // 初始化日志
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("NO DATABASE_URL");
    let conn = Database::connect(db_url)
        .await
        .expect("Could not connect to database");

    let _ = crate::db::create_user_table(&conn).await;
    let _ = crate::db::create_file_table(&conn).await;

    let app = Router::new()
        .route("/register", post(register))
        .route("/me", get(me))
        .layer(Extension(conn));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
