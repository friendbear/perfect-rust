#![allow(dead_code)]
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Repository {
    type T; // db transaction type
    type M; // SeaORM model type

    async fn select_all(&self, db: &Self::T) -> Result<Vec<Self::M>>;
    async fn select_by_id(&self, db: &Self::T, id: i32) -> Result<Self::M>;
    async fn select_by_name_like(&self, db: &Self::T, keyword: &str) -> Result<Vec<Self::M>>;
    async fn insert(&self, db: &Self::T, model: Self::M) -> Result<Self::M>;
    async fn update_by_id(&self, db: &Self::T, model: Self::M) -> Result<Self::M>;
    async fn delete_by_id(&self, db: &Self::T, id: i32) -> Result<u64>;
}
