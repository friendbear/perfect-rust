use anyhow::Result;
use lombok::*;
use mongodb::Client;
use mongodb::Database;
//use tokio::test;
#[allow(dead_code)]
#[derive(Debug, Getter, GetterMut)]
pub struct SampleMongoClient {
    client: Client,
    database: Database,
}

impl SampleMongoClient {
    #[allow(dead_code)]
    pub async fn new(uri: &str, name: &str) -> Result<Self> {
        let client = Client::with_uri_str(uri).await?;
        let database = client.database(name);
        let instanse = Self { client, database };
        Ok(instanse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new() -> Result<()> {
        let sample_client =
            SampleMongoClient::new("mongodb://localhost:27017", "rust_sample").await;
        match sample_client {
            Ok(client) => {
                println!("{:?}", client.get_client());
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
        Ok(())
    }
}
