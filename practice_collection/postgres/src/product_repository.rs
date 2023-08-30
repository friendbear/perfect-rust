use postgres::Transaction;
use postgres::types::Type;
use anyhow::{Result, Error};
use crate::repository::Repository;
use crate::entities::Product;


/// productテーブルにアクセスするRepository
pub struct ProductRepository<'a, 'b>(pub &'a mut Transaction<'b>);
impl Repository<Product, i32, u64> for ProductRepository<'_, '_> {

    fn select_all(&mut self) -> Result<Vec<Product>> {
        let sql = r#"
            SELECT id, name, price, category_id FROM product 
        "#;

        let rows = self.0.query(sql, &[])?;
        let mut products = Vec::<Product>::new();
        for row in rows {
            products.push(
                Product::new(
                    row.get("id"),
                    row.get("name"),
                    row.get("price"),
                    None
                )
            );
        }
        Ok(products)
    }
    fn select_by_id(&mut self, id: i32) -> Result<Product> {
        let sql = r#"
            SELECT id, name, price, category_id FROM product WHERE id =$1 
        "#;
        // プレースフォルダの型設定
        let stmt = self.0.prepare_typed(sql, &[Type::INT4])?; // 4byte
        let result = self.0.query_opt(&stmt, &[&id])?;
        match result {
            Some(row) => {
                Ok(Product::new(
                    row.get("id"),
                    row.get("name"),
                    row.get("price"),
                    None
                ))
            },
            None => Err(Error::msg(
                format!("Not found id:{}", id)
            )),
        }
    }
    fn update_by_id(&mut self, _id: i32) -> Result<u64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::{params::ConnectParams, transaction::TransactionUtil};
    use crate::connect::PostgresSampleClient;

    use super::*;

    fn connection_params() -> ConnectParams {
        let params = ConnectParams::new(
            "localhost".to_owned(),
            5432,
            "rust_sample".to_owned(),
            "postgres".to_owned(),
            "admin".to_owned()
        );
        params
    }
    #[test]
    fn test_select_all() -> anyhow::Result<()> {

        let mut client = PostgresSampleClient::config_connect(connection_params())?;
        let mut transaction = TransactionUtil::start(&mut client, true)?;
        let mut repository = ProductRepository(&mut transaction);
        let result = repository.select_all()?;
        println!("{:?}", result);
        Ok(())
    }

    #[test]
    fn test_select_by_id() -> anyhow::Result<()> {
        let mut client = PostgresSampleClient::config_connect(connection_params())?;
        let mut transaction = TransactionUtil::start(&mut client, true)?;
        let mut repository = ProductRepository(&mut transaction);
        let result = repository.select_by_id(1);
        match result {
            Ok(product) => println!("{:?}", product),
            Err(_e) => unreachable!(),
        }
        let result = repository.select_by_id(-1);
        match result {
            Ok(_product) => panic!("test failed."),
            Err(e) => println!("{:?}", e.to_string()),
        }

        Ok(())
    }
}