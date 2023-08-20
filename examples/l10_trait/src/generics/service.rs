use super::traits::{CsvReader, JsonReader};
use super::traits_impl::{CsvReaderImpl, JsonReaderImpl};
use anyhow::Result;
use serde::de::DeserializeOwned;

pub struct ReadService<T> {
    // CsvReaderトレイトの実装型フィールド
    csv_reader: Box<dyn CsvReader<T>>,
    json_reader: Box<dyn JsonReader<T>>,
}

impl<T> ReadService<T>
where
    T: DeserializeOwned + 'static,
{
    pub fn new() -> Self {
        Self {
            csv_reader: Box::new(CsvReaderImpl::<T>::new()) as Box<dyn CsvReader<T>>,
            json_reader: Box::new(JsonReaderImpl::<T>::new()) as Box<dyn JsonReader<T>>,
        }
    }

    pub fn csv_reader(&self, file_path: &str) -> Result<Vec<T>> {
        self.csv_reader.read(file_path)
    }

    pub fn json_reader(&self, file_path: &str) -> Result<Vec<T>> {
        self.json_reader.read(file_path)
    }
}
