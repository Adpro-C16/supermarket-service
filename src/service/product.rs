use sqlx::PgPool;

use crate::model::product::{CreateProductDto, Product, UpdateProductDto};
use crate::repository::product::ProductRepository;
use rocket::log;

pub struct ProductService {}

impl ProductService {
    pub async fn find_all(db_pool: PgPool) -> Option<Vec<Product>> {
        let products = ProductRepository::find_all(db_pool).await;
        match products {
            Some(products) => {
                log::info_!("retrieved {} products", products.len());
                Some(products)
            }
            None => {
                log::error_!("Failed to get all products");
                None
            }
        }
    }

    pub async fn find_by_id(db_pool: PgPool, id: i32) -> Option<Product> {
        let product = ProductRepository::find_by_id(db_pool, id).await;
        match product {
            Some(product) => {
                log::info_!(
                    "retrieved product | id={}, name={}, price={}, supermarket_id={}",
                    product.product_id,
                    product.product_name,
                    product.product_price,
                    product.supermarket_id
                );
                Some(product)
            }
            None => {
                log::warn_!("Product not found");
                None
            }
        }
    }

    pub async fn create(db_pool: PgPool, product: CreateProductDto) -> Option<Product> {
        let product = ProductRepository::create(db_pool, product).await;
        match product {
            Some(product) => {
                log::info_!(
                    "created product | id={}, name={}, price={}, supermarket_id={}",
                    product.product_id,
                    product.product_name,
                    product.product_price,
                    product.supermarket_id
                );
                Some(product)
            }
            None => {
                log::error_!("Failed to create product");
                None
            }
        }
    }

    pub async fn update(db_pool: PgPool, id: i32, product: UpdateProductDto) -> Option<Product> {
        let product = ProductRepository::update(db_pool, id, product).await;
        log::info_!("updated product | id={}", id);
        match product {
            Some(product) => {
                log::info_!(
                    "updated product | id={}, name={}, price={}, supermarket_id={}",
                    product.product_id,
                    product.product_name,
                    product.product_price,
                    product.supermarket_id
                );
                Some(product)
            }
            None => None,
        }
    }

    pub async fn delete(db_pool: PgPool, id: i32) -> Option<Product> {
        let product = ProductRepository::delete(db_pool, id).await;
        match product {
            Some(product) => {
                log::info_!(
                    "deleted product | id={}, name={}, price={}, supermarket_id={}",
                    product.product_id,
                    product.product_name,
                    product.product_price,
                    product.supermarket_id
                );
                Some(product)
            }
            None => {
                log::error_!("Failed to delete product");
                None
            }
        }
    }
}
