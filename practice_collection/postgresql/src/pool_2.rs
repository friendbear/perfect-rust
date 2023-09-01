use once_cell::sync::Lazy;
use r2d2_postgres::r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::sync::Mutex;
use std::time::Duration;

#[allow(dead_code)]
// connection str
static CONNECTION_STR: &str =
    "host=localhost port=5432 dbname=rust_sample user=postgres password=admin";

#[allow(dead_code)]
// create connection pool
pub static SAMPLE_POOL_2: Lazy<Mutex<Pool<PostgresConnectionManager<NoTls>>>> = Lazy::new(|| {
    let config = CONNECTION_STR.parse().unwrap();
    let manager = PostgresConnectionManager::new(config, NoTls);
    let pool = r2d2::Pool::builder()
        .max_size(30)
        .min_idle(Some(1))
        .connection_timeout(Duration::from_secs_f32(60.0))
        .build(manager)
        .unwrap();
    Mutex::new(pool)
});
