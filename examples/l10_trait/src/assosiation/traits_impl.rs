use super::traits::{CsvReader, JsonReader};
use anyhow::Result;
use csv::ReaderBuilder;
use serde::de::DeserializeOwned;
use std::{marker::PhantomData, path::PathBuf, fs::read_to_string, fs::File, io::BufReader};

#[derive(Default)]
pub struct CsvReaderImpl<T> {
    pub phantom: PhantomData<T>,
}
impl<T> CsvReader for CsvReaderImpl<T>
where
    T: DeserializeOwned,
{
    type Entity = T;

    fn read(&self, file_path: &str) -> Result<Vec<Self::Entity>> {
        let path_buff = PathBuf::from(file_path);

        let string_data = read_to_string(path_buff)?;
        let mut reader = ReaderBuilder::new().delimiter(b',').from_reader(string_data.as_bytes());
        let rows = reader.deserialize::<Self::Entity>();
        let mut result = Vec::<Self::Entity>::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }
}
#[derive(Default)]
pub struct JsonReaderImpl<T> {
    pub phantom: PhantomData<T>,
}
impl<T> JsonReader for JsonReaderImpl<T>
where
    T: DeserializeOwned,
{
    type Entity = T;

    fn read(&self, file_path: &str) -> Result<Vec<Self::Entity>> {
        let path_buff = PathBuf::from(file_path);
        let buf_reader = File::open(path_buff).map(BufReader::new)?;
        let result = serde_json::from_reader::<BufReader<File>, Vec<Self::Entity>>(buf_reader)?;
        Ok(result)
    }
}