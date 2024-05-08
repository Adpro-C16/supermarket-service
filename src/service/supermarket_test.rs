#[cfg(test)]
mod tests {
    use crate::{
        model::supermarket::{CreateSupermarketDto, UpdateSupermarketDto},
        repository::lib::setup,
        service::supermarket::SupermarketService,
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

        let supermarket = SupermarketService::create(
            pool.clone(),
            CreateSupermarketDto {
                name: "Supermarket".to_string(),
                manager_id: 1,
            },
        )
        .await
        .unwrap();
        assert_eq!(supermarket.name, "Supermarket");
        assert_eq!(supermarket.balance, 0);

        let result = sqlx::query!(
            "SELECT id, name, balance, manager_id FROM supermarket WHERE name = 'Supermarket'"
        )
        .fetch_one(&mut *conn)
        .await
        .unwrap();
        assert_eq!(result.name, "Supermarket");
        assert_eq!(result.balance, 0);
        assert_eq!(result.manager_id, 1);
    }

    #[sqlx::test]
    async fn test_create_supermarket_invalid_input() {
        let pool = setup().await;
        let result = SupermarketService::create(
            pool.clone(),
            CreateSupermarketDto {
                name: "".to_string(),
                manager_id: 5,
            },
        )
        .await;
        assert!(result.is_err());
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
            INSERT INTO supermarket (name, manager_id) VALUES ('Supermarket 1', 1);
            "#,
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        sqlx::query!(
            r#"
            INSERT INTO supermarket (name, manager_id) VALUES ('Supermarket 2', 1);
            "#,
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        let supermarkets = SupermarketService::get_all_supermarkets(pool.clone()).await;
        assert_eq!(supermarkets.is_ok(), true);
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
            INSERT INTO supermarket (name, manager_id) VALUES ('Supermarket 1', 1);
            "#,
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        let supermarket = SupermarketService::find_by_id(pool.clone(), 1)
            .await
            .unwrap();
        assert_eq!(supermarket.name, "Supermarket 1");
    }

    #[sqlx::test]
    async fn test_get_supermarket_by_id_not_found() {
        let pool = &setup().await;
        let result = SupermarketService::find_by_id(pool.clone(), 100).await;
        assert!(result.is_err());
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
            INSERT INTO supermarket (name, manager_id) VALUES ('Supermarket 1', 1);
            "#,
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        let sup = sqlx::query!(
            r#"
            SELECT id FROM supermarket WHERE name = 'Supermarket 1';
            "#,
        )
        .fetch_one(&mut *conn)
        .await
        .unwrap();

        let supermarket = SupermarketService::update(
            pool.clone(),
            sup.id,
            UpdateSupermarketDto {
                name: Some("Supermarket 2".to_string()),
                balance: Some(1000),
                manager_id: Some(2),
            },
        )
        .await
        .unwrap();
        assert_eq!(supermarket.name, "Supermarket 2");
        assert_eq!(supermarket.balance, 1000);
        assert_eq!(supermarket.manager_id, 2);
    }

    #[sqlx::test]
    async fn test_update_supermarket_not_found() {
        let pool = &setup().await;
        let result = SupermarketService::update(
            pool.clone(),
            100,
            UpdateSupermarketDto {
                name: Some("Supermarket 2".to_string()),
                balance: Some(1000),
                manager_id: Some(2),
            },
        )
        .await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_update_supermarket_invalid_input() {
        let pool = &setup().await;
        let result = SupermarketService::update(
            pool.clone(),
            1,
            UpdateSupermarketDto {
                name: Some("".to_string()),
                balance: Some(1000),
                manager_id: Some(2),
            },
        )
        .await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_delete_supermarket_not_found() {
        let pool = &setup().await;
        let result = SupermarketService::delete(pool.clone(), 100).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_delete_supermarket() {
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
            INSERT INTO supermarket (name, manager_id) VALUES ('Supermarket 1', 1);
            "#,
        )
        .execute(&mut *conn)
        .await
        .unwrap();
        let sup = sqlx::query!(
            r#"
            SELECT id FROM supermarket WHERE name = 'Supermarket 1';
            "#,
        )
        .fetch_one(&mut *conn)
        .await
        .unwrap();

        let supermarket = SupermarketService::delete(pool.clone(), sup.id)
            .await
            .unwrap();
        assert_eq!(supermarket.name, "Supermarket 1");
    }
}
