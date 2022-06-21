use sea_orm::{ConnectionTrait, DbConn, EntityTrait, ExecResult, Schema, Statement};
use tracing::debug;

use crate::http::error::Error;

pub mod users;

async fn create_table(entity: impl EntityTrait, conn: &DbConn) -> anyhow::Result<ExecResult> {
    let builder = conn.get_database_backend();
    let schema = Schema::new(builder);
    let stmt: Statement = builder.build(schema.create_table_from_entity(entity).if_not_exists());
    Ok(conn.execute(stmt).await.map_err(|e| {
        debug!("init table error {:?}", e);
        Error::SeaOrm(e)
    })?)
}

pub async fn create_user_table(conn: &DbConn) -> anyhow::Result<ExecResult> {
    Ok(create_table(crate::db::users::Entity, conn).await?)
}
