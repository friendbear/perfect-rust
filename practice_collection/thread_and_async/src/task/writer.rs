use anyhow::{Context, Result};
use async_std::channel::Receiver;
use csv::WriterBuilder;
use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;
use std::marker::PhantomData;

/// Serialize CSV

pub struct SampleWriter<T> {
    phantom: PhantomData<T>,
}
impl<T> Default for SampleWriter<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
impl<T> SampleWriter<T>
where
    T: Serialize,
{
    /// タスク間通信
    /// 受信した顧客情報をCSVファイルに格納する
    ///
    pub async fn csv_writer(&self, receiver: Receiver<T>) -> Result<String> {
        let mut receive_data = Vec::<T>::new();
        //loop {
        let recv = receiver.recv().await?;
        receive_data.push(recv);
        //    break;
        //}
        let path = std::env::current_dir().map(|path_buf| path_buf.join("resources/type_t.csv"))?;
        dbg!(&path);
        let mut writer = WriterBuilder::new().from_path(path)?;
        for data in receive_data {
            writer.serialize::<T>(data)?;
        }
        writer.flush().context("Failed to flush CSV writer")?;
        Ok(String::from("csv writer finished"))
    }
    pub async fn json_writer(&self, receiver: Receiver<T>) -> Result<String> {
        let mut receive_data = Vec::<T>::new();
        //loop {
        let recv = receiver.recv().await?;
        receive_data.push(recv);
        //    break;
        //}
        let path =
            std::env::current_dir().map(|path_buf| path_buf.join("resources/type_t.json"))?;
        dbg!(&path);
        let writer = File::create(path).map(BufWriter::new)?;
        serde_json::to_writer(writer, &receive_data)?;
        Ok(String::from("json writer finished"))
    }
}
