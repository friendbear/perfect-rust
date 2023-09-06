use anyhow::Result;
use dotenv::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;

#[allow(dead_code)]
pub struct SamplePool;
impl SamplePool {
    #[allow(dead_code)]
    pub async fn get() -> Result<DatabaseConnection> {
        dotenv().ok(); // convert Result to Option
        let database_url = env::var("DATABASE_URL")?;
        let mut options = ConnectOptions::new(database_url);
        options
            .max_connections(10)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(10))
            .max_lifetime(Duration::from_secs(10))
            .sqlx_logging(true);

        let conn = Database::connect(options).await?;
        Ok(conn)
    }
}
