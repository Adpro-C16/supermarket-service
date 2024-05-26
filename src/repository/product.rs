use crate::model::product::{CreateProductDto, Product, UpdateProductDto};
use sqlx::PgPool;

pub struct ProductRepository {}

impl ProductRepository {
    pub async fn find_all(db_pool: PgPool) -> Option<Vec<Product>> {
        let query = sqlx::query_as!(
            Product,
            r#"
            SElECT product_id, product_name, product_price, supermarket_id
            FROM product
            "#,
        )
        .fetch_all(&db_pool)
        .await;

        match query {
            Ok(supermarkets) => Some(supermarkets),
            Err(_) => None,
        }
    }

    pub async fn find_by_id(db_pool: PgPool, id: i32) -> Option<Product> {
        let query = sqlx::query_as!(
            Product,
            r#"
                SELECT product_id, product_name, product_price, supermarket_id
                FROM product
                WHERE product_id = $1
                "#,
            id
        )
        .fetch_one(&db_pool)
        .await;

        match query {
            Ok(product) => Some(product),
            Err(_) => None,
        }
    }

    pub async fn create(db_pool: PgPool, product: CreateProductDto) -> Option<Product> {
        let query = sqlx::query_as!(
            Product,
            r#"
            INSERT INTO product (product_name, product_price, supermarket_id)
            VALUES ($1, $2, $3)
            RETURNING product_id, product_name, product_price, supermarket_id
            "#,
            product.product_name,
            product.product_price,
            product.supermarket_id
        )
        .fetch_one(&db_pool)
        .await;

        match query {
            Ok(product) => Some(product),
            Err(_) => None,
        }
    }

    pub async fn update(db_pool: PgPool, id: i32, product: UpdateProductDto) -> Option<Product> {
        let query = sqlx::query_as!(
            Product,
            r#"
            UPDATE product
            SET product_name = COALESCE($1, product_name),
                product_price = COALESCE($2, product_price)
            WHERE product_id = $3
            RETURNING product_id, product_name, product_price, supermarket_id
            "#,
            product.product_name,
            product.product_price,
            id
        )
        .fetch_one(&db_pool)
        .await;

        match query {
            Ok(product) => Some(product),
            Err(_) => None,
        }
    }

    pub async fn delete(db_pool: PgPool, id: i32) -> Option<Product> {
        let query = sqlx::query_as!(
            Product,
            r#"
            DELETE FROM product
            WHERE product_id = $1
            RETURNING product_id, product_name, product_price, supermarket_id
            "#,
            id
        )
        .fetch_one(&db_pool)
        .await;

        match query {
            Ok(product) => Some(product),
            Err(_) => None,
        }
    }
}
