use once_cell::sync::Lazy;
use r2d2_postgres::r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::sync::Mutex;

#[allow(dead_code)]
// Connecttion String
static CONNECTION_STR: &str =
    "host=localhost port=5432 dbname=rust_sample user=postgres password=admin";
#[allow(dead_code)]
// Create Connection Pool
pub static SAMPLE_POOL_1: Lazy<Mutex<Pool<PostgresConnectionManager<NoTls>>>> = Lazy::new(|| {
    let config = CONNECTION_STR.parse().unwrap();
    // create connection manager
    let manager = PostgresConnectionManager::new(config, NoTls);
    // create pool
    let pool = r2d2::Pool::new(manager).unwrap();
    Mutex::new(pool)
});
