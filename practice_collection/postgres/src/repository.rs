use anyhow::Result;

pub trait Repository<T,PK, UPD> {
    fn select_all(&mut self) -> Result<Vec<T>> {
        todo!()
    }
    fn select_by_id(&mut self, _id: PK) -> Result<T> {
        todo!()
    }
    fn insert(&mut self, _row: T) -> Result<UPD> {
        todo!()
    }
    fn update_by_id(&mut self, _id: PK) -> Result<UPD> {
        todo!()
    }
    fn delete(&mut self, _id: PK) -> Result<UPD> {
        todo!()
    }
}