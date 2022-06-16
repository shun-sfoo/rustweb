use std::sync::Arc;

use sea_orm::DbConn;

use crate::config::Config;

struct ApiContext {
    config: Arc<Config>,
    db: DbConn,
}
