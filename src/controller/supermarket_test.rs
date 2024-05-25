#[cfg(test)]
mod tests {
    use crate::{controller::supermarket::find_all, repository::lib::setup};
    use rocket::{tokio, State};

    #[tokio::test]
    async fn test_find_all_supermarkets() {
        let pool = setup().await;
        let response = find_all(State::from(&pool)).await;
        assert_eq!(response.is_err(), false);
    }
}
