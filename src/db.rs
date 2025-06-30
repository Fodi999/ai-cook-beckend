use sqlx::{PgPool, Pool, Postgres};
use tracing::{info, instrument};

pub type DbPool = Pool<Postgres>;

#[instrument]
pub async fn init_db(database_url: &str) -> Result<DbPool, sqlx::Error> {
    info!("Connecting to database...");
    
    let pool = PgPool::connect(database_url).await?;
    
    info!("Database connection established");
    Ok(pool)
}
