use crate::model::supermarket::{CreateSupermarketDto, Supermarket, UpdateSupermarketDto};
use rocket::serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserData {
    pub id: i64,
    pub username: String,
    pub email: String,
}

pub struct SupermarketRepository {}

impl SupermarketRepository {
    pub async fn find_all(db_pool: PgPool) -> Option<Vec<Supermarket>> {
        let query = sqlx::query_as!(
            Supermarket,
            r#"
            SELECT id, name, balance, manager_id, created_at::text
            FROM supermarket
            "#,
        )
        .fetch_all(&db_pool)
        .await;

        match query {
            Ok(supermarkets) => Some(supermarkets),
            Err(_) => None,
        }
    }

    pub async fn find_by_id(db_pool: PgPool, id: i64) -> Option<Supermarket> {
        let query = sqlx::query_as!(
            Supermarket,
            r#"
                SELECT id, name, balance, manager_id, created_at::text
                FROM supermarket
                WHERE id = $1
                "#,
            id
        )
        .fetch_one(&db_pool)
        .await;

        match query {
            Ok(supermarket) => Some(supermarket),
            Err(_) => None,
        }
    }

    pub async fn create(db_pool: PgPool, supermarket: CreateSupermarketDto) -> Option<Supermarket> {
        let query = sqlx::query_as!(
                Supermarket,
                "INSERT INTO supermarket (name, manager_id) VALUES ($1, $2) RETURNING id, name, balance, manager_id, created_at::text",
                supermarket.name,
                supermarket.manager_id
            )
            .fetch_one(&db_pool)
            .await;

        match query {
            Ok(supermarket) => Some(supermarket),
            Err(_) => None,
        }
    }

    pub async fn update(
        db_pool: PgPool,
        id: i64,
        supermarket: UpdateSupermarketDto,
    ) -> Option<Supermarket> {
        let query = sqlx::query_as!(
            Supermarket,
            r#"
            UPDATE supermarket
            SET name = COALESCE($2, name),
                balance = COALESCE($3, balance),
                manager_id = COALESCE($4, manager_id)
            WHERE id = $1
            RETURNING id, name, balance, manager_id, created_at::text
            "#,
            id,
            supermarket.name,
            supermarket.balance,
            supermarket.manager_id
        )
        .fetch_one(&db_pool)
        .await;

        match query {
            Ok(supermarket) => Some(supermarket),
            Err(_) => None,
        }
    }

    pub async fn delete(db_pool: PgPool, id: i64) -> Option<Supermarket> {
        let query = sqlx::query_as!(
            Supermarket,
            r#"
            DELETE FROM supermarket
            WHERE id = $1
            RETURNING id, name, balance, manager_id, created_at::text
            "#,
            id
        )
        .fetch_one(&db_pool)
        .await;

        match query {
            Ok(supermarket) => Some(supermarket),
            Err(_) => None,
        }
    }

    pub async fn list_all_users(db_pool: PgPool) -> Option<Vec<UserData>> {
        let query = sqlx::query_as!(
            UserData,
            r#"
            SELECT id, username, email
            FROM users
            WHERE NOT role = 'Admin'
            "#,
        )
        .fetch_all(&db_pool)
        .await;

        match query {
            Ok(users) => Some(users),
            Err(_) => None,
        }
    }
}
