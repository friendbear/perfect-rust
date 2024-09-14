use crate::connect::SampleMongoClient;
use crate::entites::Product;
use crate::repository::Repository;
use anyhow::{Error, Result};
use async_trait::async_trait;
use futures_util::StreamExt;
use mongodb::bson::doc;
use mongodb::options::UpdateModifications;
use mongodb::Collection;

pub struct ProductRepository {
    collection: Collection<Product>,
}

impl ProductRepository {
    #[allow(dead_code)]
    pub fn new(client: SampleMongoClient, collection_name: &str) -> Self {
        Self {
            collection: client.get_database().collection(collection_name),
        }
    }
}

#[async_trait]
impl Repository<Product, i32, bool> for ProductRepository {
    async fn select_all(&self) -> Result<Vec<Product>> {
        //        let doc = doc! {"price", 1 };
        //        let options = FindOptions::builder().sort(doc).build();
        let mut cursor = self.collection.find(doc! {}).await?;
        let mut products = Vec::new();
        while let Some(product) = cursor.next().await {
            products.push(product?);
        }
        Ok(products)
    }
    async fn select_by_id(&self, id: i32) -> Result<Product> {
        self.collection
            .find_one(doc! {"product_id": id})
            .await?
            .ok_or(Error::msg("Not found"))
    }
    async fn insert(&self, row: Product) -> Result<bool> {
        self.collection
            .insert_one(row)
            .await
            .map(|_| Ok(true))?
    }
    async fn insert_many(&self, cols: Vec<Product>) -> Result<bool> {
        self.collection
            .insert_many(cols.clone())
            .await
            .map(|result| {
                if result.inserted_ids.iter().len() == cols.iter().len() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            })?
    }
    async fn update_by_id(&self, id: i32, row: Product) -> Result<bool> {
        let query = doc! { "product_id": id };
        let update = UpdateModifications::Document(
            doc! { "name": row.get_name(), "price": row.get_price() },
        );
        self.collection
            .update_one(query, update)
            .await
            .map(|result| {
                if result.modified_count == 1 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            })?
    }

    async fn delete_by_id(&self, id: i32) -> Result<bool> {
        let query = doc! { "product_id": id };
        self.collection
            .delete_one(query)
            .await
            .map(|result| {
                if result.deleted_count == 0 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            })?
    }
    async fn count_documents(&self) -> Result<u64> {
        self.count_documents().await
    }
}
