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
        let params = CONNECT_PARAMS.lock().unwrap();
        let (client, connection) =
            tokio_postgres::connect(params.connect_string().as_str(), NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                panic!("{}", e);
            }
        });
        Ok(client)
    }
}
