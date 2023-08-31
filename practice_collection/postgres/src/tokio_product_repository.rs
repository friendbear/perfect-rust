use crate::entities::Product;
use crate::tokio_repository::AsyncRepository;
use anyhow::Result;
use async_trait::async_trait;
use tokio_postgres::Transaction;

pub struct ProductRepository<'a, 'b> {
    transaction: &'a mut Transaction<'b>,
}
impl<'a, 'b> ProductRepository<'a, 'b> {
    #[allow(dead_code)]
    pub fn new(tran: &'a mut Transaction<'b>) -> Self {
        Self { transaction: tran }
    }
}
#[async_trait]
impl AsyncRepository<Product, i32, bool> for ProductRepository<'_, '_> {
    async fn select_all(&mut self) -> Result<Vec<Product>> {
        let sql = "SELECT id, name, price, category_id FROM product";
        let rows = self.transaction.query(sql, &[]).await?;
        let mut products = Vec::<Product>::new();
        for row in rows {
            products.push(Product::new(
                row.get(0),
                row.get(1),
                row.get(2),
                row.get(3),
                None,
            ));
        }
        Ok(products)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tokio_connect::AsyncSimpleClient;
    use crate::tokio_transaction::AsyncTransactionUtil;
    use anyhow::Result;

    #[ignore = "Need database access"]
    #[tokio::test]
    async fn test_select_all() -> Result<()> {
        let mut client = AsyncSimpleClient::connect().await?;
        let mut transaction = AsyncTransactionUtil::start(&mut client, true).await?;
        let mut repository = ProductRepository::new(&mut transaction);
        let products = repository.select_all().await?;
        dbg!(&products);
        Ok(())
    }
}
