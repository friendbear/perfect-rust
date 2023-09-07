use crate::modules::prelude::*;
use crate::modules::*;
use anyhow::Result;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseTransaction;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
pub struct ProductCategoryRepository;
impl ProductCategoryRepository {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }
    #[allow(dead_code)]
    async fn select_by_id_join_product(
        &self,
        db: &DatabaseTransaction,
        id: i32,
    ) -> Result<Vec<(product_category::Model, Vec<product::Model>)>> {
        let products = ProductCategory::find_by_id(id)
            .find_with_related(Product)
            .filter(product::Column::Name.contains("は"))
            .all(db)
            .await?;
        Ok(products)
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::TransactionTrait;

    use crate::pool::SamplePool;

    use super::*;
    #[ignore = "Need Database connection."]
    #[tokio::test]
    async fn test_select_by_id_join_product() -> Result<()> {
        let pool = SamplePool::get().await?;
        let db = pool.begin().await?;

        let result = ProductCategoryRepository::new()
            .select_by_id_join_product(&db, 5)
            .await?;

        for row in result {
            println!("{row:?}");
        }
        Ok(())
    }
}
