use crate::modules::product::{Column, Entity as Product, Model};
use crate::repository::Repository;
use anyhow::{Error, Result};
use async_trait::async_trait;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set; // {NotSet, Set, Unchanged};
use sea_orm::ColumnTrait;
use sea_orm::DatabaseTransaction;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;

pub struct ProductRepository;
impl ProductRepository {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Repository for ProductRepository {
    type T = DatabaseTransaction;
    type M = Model;

    async fn select_all(&self, db: &Self::T) -> Result<Vec<Self::M>> {
        let products = Product::find().order_by_asc(Column::Name).all(db).await?;
        Ok(products)
    }
    async fn select_by_id(&self, db: &Self::T, id: i32) -> Result<Self::M> {
        let product = Product::find_by_id(id).one(db).await?;
        match product {
            Some(v) => Ok(v),
            None => Err(Error::msg("Not found.")),
        }
    }
    async fn select_by_name_like(&self, db: &Self::T, keyword: &str) -> Result<Vec<Self::M>> {
        let products = Product::find()
            .filter(Column::Name.contains(keyword))
            .all(db)
            .await?;
        Ok(products)
    }
    async fn insert(&self, db: &Self::T, model: Self::M) -> Result<Self::M> {
        let active_model = model.into_active_model();
        let result = active_model.insert(db).await?;
        Ok(result)
    }
    async fn update_by_id(&self, db: &Self::T, model: Self::M) -> Result<Self::M> {
        let row = Product::find_by_id(model.id).one(db).await?;
        match row {
            Some(value) => {
                let mut update_active_model = value.into_active_model();
                // update active model
                update_active_model.name = Set(model.name);
                update_active_model.price = Set(model.price);
                update_active_model.category_id = Set(model.category_id);
                let result = update_active_model.update(db).await?;
                Ok(result)
            }
            None => Err(Error::msg("No record to update.")),
        }
    }
    async fn delete_by_id(&self, db: &Self::T, id: i32) -> Result<u64> {
        let row = Product::find_by_id(id).one(db).await?;
        match row {
            Some(value) => {
                let delete_active_model = value.into_active_model();
                // update active model
                let result = delete_active_model.delete(db).await?;
                Ok(result.rows_affected)
            }
            None => Err(Error::msg("No record to delete.")),
        }
    }
}

use crate::modules::prelude::*;
use crate::modules::{product, product_category};
impl ProductRepository {
    #[allow(dead_code)]
    async fn select_by_id_join_productt_category(
        &self,
        db: &DatabaseTransaction,
        id: i32,
    ) -> Result<Vec<(product::Model, Option<product_category::Model>)>> {
        let product_and_category = Product::find_by_id(id)
            .find_also_related(ProductCategory)
            .all(db)
            .await?;
        Ok(product_and_category)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pool::SamplePool;
    use sea_orm::TransactionTrait;

    #[ignore = "Need Database connection."]
    #[tokio::test]
    async fn test_select_all() -> Result<()> {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
        let pool = SamplePool::get().await?;
        let db = pool.begin().await?;
        let rows = ProductRepository::new().select_all(&db).await?;
        for row in rows {
            println!("{row:?}");
        }
        Ok(())
    }
    #[tokio::test]
    async fn test_select_by_id_join_product_category() -> Result<()> {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
        let pool = SamplePool::get().await?;
        let db = pool.begin().await?;
        let rows = ProductRepository::new()
            .select_by_id_join_productt_category(&db, 1)
            .await?;
        for row in rows {
            println!("{row:?}");
        }
        Ok(())
    }
    #[ignore = "Need Database connection."]
    #[tokio::test]
    async fn test_select_by_id() -> Result<()> {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
        let pool = SamplePool::get().await?;
        let db = pool.begin().await?;
        let row = ProductRepository::new().select_by_id(&db, 1).await?;
        println!("{row:?}");
        Ok(())
    }
    #[ignore = "Need Database connection."]
    #[tokio::test]
    async fn test_select_by_like_name() -> Result<()> {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
        let pool = SamplePool::get().await?;
        let db = pool.begin().await?;
        let rows = ProductRepository::new()
            .select_by_name_like(&db, "も")
            .await?;
        for row in rows {
            println!("{row:?}");
        }
        Ok(())
    }
    #[ignore = "Need Database connection."]
    #[tokio::test]
    async fn test_insert() -> Result<()> {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
        let model: Model = Model {
            id: 0,
            name: "はしっこ".to_owned(),
            price: 1000,
            category_id: Some(5),
        };
        let pool = SamplePool::get().await?;
        let db = pool.begin().await?;
        let insert_result = ProductRepository::new().insert(&db, model).await?;
        println!("{insert_result:?}");
        db.rollback().await?;
        Ok(())
    }
    #[ignore = "Need Database connection."]
    #[tokio::test]
    async fn test_update_by_id() -> Result<()> {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
        let model: Model = Model {
            id: 11,
            name: "花ノ木もえ".to_owned(),
            price: 10000,
            category_id: Some(5),
        };
        let pool = SamplePool::get().await?;
        let db = pool.begin().await?;
        let update_result = ProductRepository::new().update_by_id(&db, model).await?;
        println!("{update_result:?}");
        db.commit().await?;
        Ok(())
    }
    #[ignore = "Need Database connection."]
    #[tokio::test]
    async fn test_delete_by_id() -> Result<()> {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
        let pool = SamplePool::get().await?;
        let db = pool.begin().await?;
        let row = ProductRepository::new().delete_by_id(&db, 1).await?;
        println!("{row:?}");
        db.rollback().await?;
        Ok(())
    }
}
