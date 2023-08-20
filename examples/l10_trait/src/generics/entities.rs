use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: u32,
}
