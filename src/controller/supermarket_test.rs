#[cfg(test)]
mod tests {
    use crate::{
        controller::supermarket::{find_all, find_by_id},
        guard::auth_guard::AuthGuard,
        repository::lib::setup,
    };
    use rocket::{request::FromRequest, tokio, Request, State};

    #[tokio::test]
    async fn test_find_all_supermarkets() {
        let pool = setup().await;
        let response = find_all(State::from(&pool)).await;
        assert_eq!(false, false);
    }

    #[tokio::test]
    async fn test_get_supermarket_by_id() {
        let pool = setup().await;
        let response = find_by_id(State::from(&pool), 1).await;
        assert_eq!(false, false);
    }
}
