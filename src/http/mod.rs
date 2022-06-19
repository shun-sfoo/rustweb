pub mod error;
mod users;

use anyhow::Context;
use std::sync::Arc;

use axum::{Extension, Router};
use sea_orm::DbConn;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::config::Config;

use self::error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    db: DbConn,
}

pub async fn serve(config: Config, db: DbConn) -> anyhow::Result<()> {
    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(Extension(ApiContext {
                config: Arc::new(config),
                db,
            }))
            // Enables logging. Use `RUST_LOG=tower_http=debug`
            .layer(TraceLayer::new_for_http()),
    );

    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router {
    users::router()
}
