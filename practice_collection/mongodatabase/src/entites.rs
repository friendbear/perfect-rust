use lombok::*;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(
    Getter,
    GetterMut,
    Setter,
    NoArgsConstructor,
    AllArgsConstructor,
    Clone,
    Debug,
    Deserialize,
    Serialize,
)]
pub struct ProductCategory {
    category_id: i32,
    name: String,
}

#[derive(
    Getter,
    GetterMut,
    Setter,
    NoArgsConstructor,
    AllArgsConstructor,
    Clone,
    Debug,
    Deserialize,
    Serialize,
)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    product_id: i32,
    name: String,
    price: i32,
    category: Option<ProductCategory>,
}
