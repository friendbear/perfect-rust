use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use r2d2::PooledConnection;
use anyhow::Result;
use crate::pool_1::SAMPLE_POOL_1;
/// ConnectionManager
pub struct SamplePoolManager;
impl SamplePoolManager {
    #[allow(dead_code)]
    pub fn client() -> Result<PooledConnection<PostgresConnectionManager<NoTls>>> {
        // Get lock and pool
        let pool = SAMPLE_POOL_1.lock().unwrap();
        println!("state: {:?}", pool.state());
        // get connection from pool
        let client = pool.get()?;
        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use crate::repository::Repository;
    use crate::transaction::TransactionUtil;
    use crate::product_repository::ProductRepository;

    #[test]
    fn test_use_connection_pool() -> Result<()> {
        let handle = thread::spawn(|| {
            let mut client = SamplePoolManager::client()?;
            let mut transaction = TransactionUtil::start(&mut client, true)?;
            let mut repository = ProductRepository(&mut transaction);
            let product = repository.select_by_id(1);
            product
        });
        let product = handle.join().unwrap().unwrap();
        dbg!(&product);
        assert_eq!(product.get_name(), "はしちゃん");
        Ok(())
    }
}