use super::traits::{CsvReader, JsonReader};
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub struct CsvReaderImpl<T> {
    pub phantom: PhantomData<T>,
}
impl<T> CsvReader<T> for CsvReaderImpl<T>
where
    T: DeserializeOwned,
{
    fn read(&self, file_path: &str) -> Result<Vec<T>> {
        todo!()
    }
}

pub struct JsonReaderImpl<T> {
    pub phantom: PhantomData<T>,
}
impl<T> JsonReader<T> for JsonReaderImpl<T>
where
    T: DeserializeOwned,
{
    fn read(&self, file_path: &str) -> Result<Vec<T>> {
        todo!()
    }
}
