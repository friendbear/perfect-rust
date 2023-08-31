use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait AsyncRepository<T, PK, UPD> {
    async fn select_all(&mut self) -> Result<Vec<T>>;
    //async fn select_by_id(&mut self, id: PK) -> Result<T>;
    //async fn insert(&mut self, row: T) -> Result<()>;
    //async fn update_by_id(&mut self, id: PK) -> Result<UPD>;
    //async fn delete_by_id(&mut self, id: PK) -> Result<UPD>;
}
