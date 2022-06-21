use clap::Parser;
use ruthenium::{
    config::Config,
    db,
    http::{self, error::Error},
};
use sea_orm::Database;
use tracing::debug;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    let db = Database::connect(&config.database_url).await.map_err(|e| {
        debug!("failed to connect to database: {:?}", e);
        Error::SeaOrm(e)
    })?;

    db::create_user_table(&db).await?;

    http::serve(config, db).await?;

    Ok(())
}
