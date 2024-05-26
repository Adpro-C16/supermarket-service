use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    pub product_id: i32,
    pub product_name: String,
    pub product_price: i32,
    pub supermarket_id: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateProductDto {
    pub product_name: String,
    pub product_price: i32,
    pub supermarket_id: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateProductDto {
    pub product_name: Option<String>,
    pub product_price: Option<i32>,
}
