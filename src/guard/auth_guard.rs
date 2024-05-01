use std::env;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

pub mod services {
    tonic::include_proto!("services");
}

use services::{auth_service_client::AuthServiceClient, VerifyRequest};

use crate::library::error_response::ErrorResponse;

#[derive(Debug)]
pub struct AuthGuard {
    pub success: bool,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ErrorResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ErrorResponse> {
        let auth_header = req.headers().get_one("Authorization");
        if auth_header.is_none() {
            return Outcome::Error((
                Status::Unauthorized,
                ErrorResponse {
                    status_code: rocket::http::Status::Unauthorized,
                    message: "Unauthorized".to_string(),
                },
            ));
        }
        let token = auth_header.unwrap().split_whitespace().last().unwrap();
        let auth_url = env::var("AUTH_SERVICE_URL").expect("AUTH_SERVICE_URL must be set.");
        let mut client = AuthServiceClient::connect(auth_url).await.unwrap();
        let request = tonic::Request::new(VerifyRequest {
            token: token.to_string(),
        });
        let response = client.verify(request).await.unwrap().into_inner();
        if response.is_valid == false {
            return Outcome::Error((
                Status::Unauthorized,
                ErrorResponse {
                    status_code: rocket::http::Status::Unauthorized,
                    message: "Unauthorized".to_string(),
                },
            ));
        }

        Outcome::Success(AuthGuard { success: true })
    }
}
