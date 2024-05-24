#[cfg(test)]
mod tests {
    use rocket::{http::Status, local::blocking::Client};

    #[test]
    fn test_get_all_supermarkets() {
        let rocket = rocket::build();
        let client = Client::tracked(rocket).unwrap();
        let response = client.get("/supermarkets").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_get_supermarket_by_id() {
        let rocket = rocket::build();
        let client = Client::tracked(rocket).unwrap();
        let response = client.get("/supermarkets/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
