use crate::params::ConnectParams;
use anyhow::Result;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tokio_postgres::{Client, NoTls};

#[allow(dead_code)]
pub static CONNECT_PARAMS: Lazy<Mutex<ConnectParams>> = Lazy::new(|| {
    let params = ConnectParams::new(
        "localhost".to_owned(),
        5432,
        "rust_sample".to_owned(),
        "postgres".to_owned(),
        "admin".to_owned(),
    );
    Mutex::new(params)
});

/// 非同期実行データベース接続機能
#[allow(dead_code)]
pub struct AsyncSimpleClient;
impl AsyncSimpleClient {
    #[allow(dead_code)]
    pub async fn connect() -> Result<Client> {
        //let config = CONNECT_PARAMS.lock().unwrap().connect_string().clone();
        let config;
        {
            let params = CONNECT_PARAMS.lock().unwrap(); // params lock here.
            config = params.connect_string().clone();
        } // params drop here.

        let (client, connection) = tokio_postgres::connect(config.as_str(), NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });
        //handle.await.expect("Async task panicked");
        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[ignore = "Need database access."]
    #[tokio::test]
    async fn test_connection() -> Result<()> {
        let _connect = AsyncSimpleClient::connect().await?;
        Ok(())
    }
}
