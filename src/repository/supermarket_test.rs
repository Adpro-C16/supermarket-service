#[cfg(test)]
mod tests {
    use crate::{
        model::supermarket::{CreateSupermarketDto, UpdateSupermarketDto},
        repository::{lib::setup, supermarket::SupermarketRepository},
    };

    #[sqlx::test]
    async fn test_create_supermarket() {
        let pool = &setup().await;
        let mut conn = pool.acquire().await.unwrap();

        sqlx::query!("ALTER SEQUENCE supermarket_id_seq RESTART WITH 1")
            .execute(&mut *conn)
            .await
            .unwrap();

        sqlx::query!("DELETE FROM supermarket")
            .execute(&mut *conn)
            .await
            .unwrap();

        let supermarket = SupermarketRepository::create(
            pool.clone(),
            CreateSupermarketDto {
                name: "Supermarket".to_string(),
            },
        )
        .await
        .unwrap();
        assert_eq!(supermarket.name, "Supermarket");
        assert_eq!(supermarket.balance, 0);

        let result =
            sqlx::query!("SELECT id, name, balance FROM supermarket WHERE name = 'Supermarket'")
                .fetch_one(&mut *conn)
                .await
                .unwrap();
        assert_eq!(result.name, "Supermarket");
        assert_eq!(result.balance, 0);
    }

    #[sqlx::test]
    async fn test_get_all_supermarkets() {
        let pool = &setup().await;
        let mut conn = pool.acquire().await.unwrap();

        sqlx::query!("ALTER SEQUENCE supermarket_id_seq RESTART WITH 1")
            .execute(&mut *conn)
            .await
            .unwrap();

        sqlx::query!("DELETE FROM supermarket")
            .execute(&mut *conn)
            .await
            .unwrap();

        sqlx::query!(
            r#"
            INSERT INTO supermarket (name) VALUES ('Supermarket 1');
            "#,
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        sqlx::query!(
            r#"
            INSERT INTO supermarket (name) VALUES ('Supermarket 2');
            "#,
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        let supermarkets = SupermarketRepository::find_all(pool.clone()).await.unwrap();
        assert_eq!(supermarkets.len(), 2);
        assert_eq!(supermarkets[0].name, "Supermarket 1");
        assert_eq!(supermarkets[1].name, "Supermarket 2");
    }

    #[sqlx::test]
    async fn test_get_supermarket_by_id() {
        let pool = &setup().await;
        let mut conn = pool.acquire().await.unwrap();

        sqlx::query!("ALTER SEQUENCE supermarket_id_seq RESTART WITH 1")
            .execute(&mut *conn)
            .await
            .unwrap();

        sqlx::query!("DELETE FROM supermarket")
            .execute(&mut *conn)
            .await
            .unwrap();

        sqlx::query!(
            r#"
            INSERT INTO supermarket (name) VALUES ('Supermarket 1');
            "#,
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        let supermarket = SupermarketRepository::find_by_id(pool.clone(), 1)
            .await
            .unwrap();
        assert_eq!(supermarket.name, "Supermarket 1");
    }

    #[sqlx::test]
    async fn test_update_supermarket() {
        let pool = &setup().await;
        let mut conn = pool.acquire().await.unwrap();

        sqlx::query!("ALTER SEQUENCE supermarket_id_seq RESTART WITH 1")
            .execute(&mut *conn)
            .await
            .unwrap();

        sqlx::query!("DELETE FROM supermarket")
            .execute(&mut *conn)
            .await
            .unwrap();

        sqlx::query!(
            r#"
            INSERT INTO supermarket (name) VALUES ('Supermarket 1');
            "#,
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        let supermarket = SupermarketRepository::update(
            pool.clone(),
            1,
            UpdateSupermarketDto {
                name: Some("Supermarket 2".to_string()),
                balance: Some(1000),
            },
        )
        .await
        .unwrap();
        assert_eq!(supermarket.name, "Supermarket 2");

        let result =
            sqlx::query!("SELECT id, name, balance FROM supermarket WHERE name = 'Supermarket 2'")
                .fetch_one(&mut *conn)
                .await
                .unwrap();
        assert_eq!(result.name, "Supermarket 2");
        assert_eq!(result.balance, 1000);
    }

    #[sqlx::test]
    async fn test_update_supermarket_name() {
        let pool = &setup().await;
        let mut conn = pool.acquire().await.unwrap();

        sqlx::query!("ALTER SEQUENCE supermarket_id_seq RESTART WITH 1")
            .execute(&mut *conn)
            .await
            .unwrap();

        sqlx::query!("DELETE FROM supermarket")
            .execute(&mut *conn)
            .await
            .unwrap();

        sqlx::query!(
            r#"
            INSERT INTO supermarket (name) VALUES ('Supermarket 1');
            "#,
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        let supermarket = SupermarketRepository::update(
            pool.clone(),
            1,
            UpdateSupermarketDto {
                name: Some("Supermarket 2".to_string()),
                balance: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(supermarket.name, "Supermarket 2");

        let result =
            sqlx::query!("SELECT id, name, balance FROM supermarket WHERE name = 'Supermarket 2'")
                .fetch_one(&mut *conn)
                .await
                .unwrap();
        assert_eq!(result.name, "Supermarket 2");
        assert_eq!(result.balance, 0);
    }

    #[sqlx::test]
    async fn test_update_supermarket_balance() {
        let pool = &setup().await;
        let mut conn = pool.acquire().await.unwrap();

        sqlx::query!("ALTER SEQUENCE supermarket_id_seq RESTART WITH 1")
            .execute(&mut *conn)
            .await
            .unwrap();

        sqlx::query!("DELETE FROM supermarket")
            .execute(&mut *conn)
            .await
            .unwrap();

        sqlx::query!(
            r#"
            INSERT INTO supermarket (name) VALUES ('Supermarket 1');
            "#,
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        let supermarket = SupermarketRepository::update(
            pool.clone(),
            1,
            UpdateSupermarketDto {
                name: None,
                balance: Some(1000),
            },
        )
        .await
        .unwrap();
        assert_eq!(supermarket.balance, 1000);

        let result =
            sqlx::query!("SELECT id, name, balance FROM supermarket WHERE name = 'Supermarket 1'")
                .fetch_one(&mut *conn)
                .await
                .unwrap();
        assert_eq!(result.name, "Supermarket 1");
        assert_eq!(result.balance, 1000);
    }
}
