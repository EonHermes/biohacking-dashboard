use sqlx::SqlitePool;
use tracing::info;

static DB_POOL: std::sync::OnceLock<SqlitePool> = std::sync::OnceLock::new();

pub async fn init_db(database_url: &str) -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect(database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    info!("Database initialized successfully");
    
    DB_POOL.set(pool).map_err(|_| sqlx::Error::Configuration(
        "Database pool already initialized".into()
    ))?;
    
    Ok(())
}

pub fn get_pool() -> &'static SqlitePool {
    DB_POOL.get().expect("Database not initialized. Call init_db first.")
}
