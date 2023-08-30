use postgres::{Client, Transaction};
use anyhow::Result;

/// トランザクション制御
pub struct TransactionUtil;
impl TransactionUtil {
    #[allow(dead_code)]
    /// トランザクションを開始する
    pub fn start(client: &mut Client, read_only: bool) -> Result<Transaction> {
        let transaction = client.build_transaction().read_only(read_only).start()?;
        Ok(transaction)
    }
    #[allow(dead_code)]
    /// トランザクションをコミットする
    pub fn commit(transaction: Transaction) -> Result<()> {
        transaction.commit()?;
        Ok(())
    }
    #[allow(dead_code)]
    /// トランザクションをロールバックする
    pub fn rollback(transaction: Transaction) -> Result<()> {
        transaction.rollback()?;
        Ok(())
    }
}