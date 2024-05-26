use sqlx::PgPool;

use crate::repository::product::{CreateProductDto, Product, ProductRepository, UpdateProductDto};

pub struct ProductService {}

impl ProductService {
    pub async fn find_all(db_pool: PgPool) -> Option<Vec<Product>> {
        let products = ProductRepository::find_all(db_pool).await;
        match products {
            Some(products) => Some(products),
            None => None,
        }
    }

    pub async fn find_by_id(db_pool: PgPool, id: i32) -> Option<Product> {
        let product = ProductRepository::find_by_id(db_pool, id).await;
        match product {
            Some(product) => Some(product),
            None => None,
        }
    }

    pub async fn create(db_pool: PgPool, product: CreateProductDto) -> Option<Product> {
        let product = ProductRepository::create(db_pool, product).await;
        match product {
            Some(product) => Some(product),
            None => None,
        }
    }

    pub async fn update(db_pool: PgPool, id: i32, product: UpdateProductDto) -> Option<Product> {
        let product = ProductRepository::update(db_pool, id, product).await;
        match product {
            Some(product) => Some(product),
            None => None,
        }
    }

    pub async fn delete(db_pool: PgPool, id: i32) -> Option<Product> {
        let product = ProductRepository::delete(db_pool, id).await;
        match product {
            Some(product) => Some(product),
            None => None,
        }
    }
}
