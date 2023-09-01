use crate::entities::{Product, ProductCategory};
use anyhow::{Error, Result};
use postgres::types::Type;
use postgres::Transaction;

#[allow(dead_code)]
pub struct ProductCategoryRepository<'a, 'b>(pub &'a mut Transaction<'b>);
impl ProductCategoryRepository<'_, '_> {
    #[allow(dead_code)]
    pub fn select_by_id_join_product(&mut self, id: i32) -> Result<ProductCategory> {
        let sql = r#"
            SELECT c.id AS c_id, c.name AS c_name, p.id, p.name, p.price, p.category_id
             FROM product_category c JOIN product p ON c.id = p.category_id WHERE c.id = $1
        "#;

        let stmt = self.0.prepare_typed(sql, &[Type::INT4])?;
        let rows = self.0.query(&stmt, &[&id])?;
        if rows.is_empty() {
            return Err(Error::msg(format!("No applicable records. id: {}", id)));
        }

        let mut product_category = ProductCategory::new(0, String::from(""), None);
        let mut products = Vec::<Product>::new();
        for row in rows {
            if product_category.get_id() == &0 {
                product_category.set_id(row.get("idc_id"));
                product_category.set_name(row.get("c_name"));
            }
            products.push(Product::new(
                row.get("id"),
                row.get("name"),
                row.get("price"),
                row.get("category_id"),
                None,
            ));
        }
        product_category.set_products(Some(products));
        Ok(product_category)
    }
}
