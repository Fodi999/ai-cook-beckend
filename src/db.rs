use sqlx::{PgPool, Pool, Postgres};
use tracing::{info, instrument};

pub type DbPool = Pool<Postgres>;

#[instrument]
pub async fn init_db(database_url: &str) -> Result<DbPool, sqlx::Error> {
    println!("🔍 init_db() started");
    println!("🔗 Attempting to connect to: {}...", &database_url[..std::cmp::min(50, database_url.len())]);
    
    info!("Connecting to database...");
    
    let pool = match PgPool::connect(database_url).await {
        Ok(pool) => {
            println!("✅ Database pool created successfully");
            pool
        },
        Err(e) => {
            println!("❌ Database connection failed: {}", e);
            return Err(e);
        }
    };
    
    println!("✅ Database connection established");
    info!("Database connection established");
    Ok(pool)
}
