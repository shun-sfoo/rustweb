use std::{env, net::SocketAddr};

use axum::{
    routing::{delete, get, post},
    Extension, Router,
};
use sea_orm::Database;
use service::{delete_file, download_file, list, login, me, register, upload, users};
use tower_http::cors::CorsLayer;

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

    let cors = CorsLayer::permissive();
    // allow `GET` and `POST` when accessing the resource
    // .allow_methods(vec![Method::GET, Method::POST])
    // allow requests from any origin
    // .allow_origin(Origin::exact("http://localhost:3000".parse().unwrap()));
    // .allow_origin(Any)
    // .allow_headers(Any);

    //TODO upload download
    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/me", get(me))
        .route("/files", get(list).delete(delete_file))
        .route("/clones", get(list).delete(delete_file))
        .route("/upload", post(upload))
        .route("/users", get(users))
        .route("/:name", get(download_file))
        .layer(Extension(conn))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
