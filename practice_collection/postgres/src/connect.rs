use postgres::{Client, NoTls, Config};
use crate::params::ConnectParams;

/// データベース接続機能の実装
#[allow(dead_code)]
pub struct PostgresSampleClient;
impl PostgresSampleClient {
    #[allow(dead_code)]
    /// Client構造体のconnect()関数を利用した接続
    pub fn simple_connect(params: ConnectParams) -> anyhow::Result<Client> {
        let client = Client::connect(&params.connect_string().as_str(), NoTls)?;
        Ok(client)
    }

    #[allow(dead_code)]
    /// Config構造体のconnect()メソッドを利用した接続
    pub fn config_connect(params: ConnectParams) -> anyhow::Result<Client> {
        let client = Config::new()
            .host(params.get_host())
            .port(params.get_port().clone())
            .dbname(params.get_dbname())
            .user(params.get_user())
            .password(params.get_password())
            .connect(NoTls)?;
        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_connect_ok() {
        let params = ConnectParams::new(
            "localhost".to_owned(),
            5432,
            "rust_sample".to_owned(),
            "postgres".to_owned(),
            "admin".to_owned()
        );
        match super::PostgresSampleClient::simple_connect(params.clone()) {
            Ok(client) => {
                println!("simple connect connection ok.");
                let _ = client.close();
            }
            Err(err) => println!("{}", err.to_string()),
        }
        match super::PostgresSampleClient::config_connect(params.clone()) {
            Ok(client) => {
                println!("config connect connection ok.");
                let _ = client.close();
            }
            Err(err) => println!("{}", err.to_string()),
        }
    }
}