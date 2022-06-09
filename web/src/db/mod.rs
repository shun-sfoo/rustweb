use sea_orm::{ConnectionTrait, DbConn, EntityTrait, ExecResult, Schema, Statement};

pub mod file;
pub mod store;
pub mod user;

async fn create_table(entity: impl EntityTrait, conn: &DbConn) -> ExecResult {
    let builder = conn.get_database_backend();
    let schema = Schema::new(builder);
    let stmt: Statement = builder.build(schema.create_table_from_entity(entity).if_not_exists());
    conn.execute(stmt).await.unwrap()
}

pub async fn create_user_table(conn: &DbConn) -> ExecResult {
    create_table(crate::db::user::Entity, conn).await
}

pub async fn create_file_table(conn: &DbConn) -> ExecResult {
    create_table(crate::db::file::Entity, conn).await
}

pub async fn create_store_table(conn: &DbConn) -> ExecResult {
    create_table(crate::db::store::Entity, conn).await
}
