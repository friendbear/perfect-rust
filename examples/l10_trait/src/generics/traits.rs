use anyhow::Result;
use serde::de::DeserializeOwned;

pub trait CsvReader<T>
where
    T: DeserializeOwned,
{
    fn read(&self, file_path: &str) -> Result<Vec<T>>;
}

pub trait JsonReader<T>
where
    T: DeserializeOwned,
{
    fn read(&self, file_path: &str) -> Result<Vec<T>>;
}
