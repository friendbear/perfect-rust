use lombok::*;

#[derive(Getter, Setter, GetterMut, NoArgsConstructor, AllArgsConstructor, ToString, Clone)]
pub struct ProductCategory {
    id: i32,
    name: String,
    products: Option<Vec<Product>>,
}

#[derive(Getter, Setter, GetterMut, NoArgsConstructor, AllArgsConstructor, ToString, Clone)]
pub struct Product {
    id: i32,
    name: String,
    price: i32,
    product_category: Option<Vec<ProductCategory>>,
}