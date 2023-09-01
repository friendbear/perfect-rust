use anyhow::Result;
use tokio_postgres::{Client, Transaction};

#[allow(dead_code)]
/// 非同期実行 トランザクション制御機能の実装
pub struct AsyncTransactionUtil;
impl AsyncTransactionUtil {
    #[allow(dead_code)]
    pub async fn start(client: &mut Client, read_only: bool) -> Result<Transaction> {
        let transaction = client
            .build_transaction()
            .read_only(read_only)
            .start()
            .await?;
        Ok(transaction)
    }
}
