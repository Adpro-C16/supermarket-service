use rocket::serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
    pub name: Option<String>,
    pub balance: Option<i32>,
    pub manager_id: Option<i32>,
}

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

    pub async fn update(
        db_pool: PgPool,
        id: i32,
        product: UpdateProductDto,
    ) -> Option<Product> {
        let query = sqlx::query_as!(
            Product,
            r#"
            UPDATE product
            SET product_name = COALESCE(product_name, $1),
                product_price = COALESCE(product_price, $2),
                supermarket_id = COALESCE(supermarket_id, $3)
            WHERE product_id = $4
            RETURNING product_id, product_name, product_price, supermarket_id
            "#,
            product.name,
            product.balance,
            product.manager_id,
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
