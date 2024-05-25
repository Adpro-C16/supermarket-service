use controller::route_stage;
use rocket::tokio;
use rocket_cors::AllowedOrigins;
use std::env;

#[macro_use]
extern crate rocket;
pub mod controller;
pub mod services {
    tonic::include_proto!("services");
}
use autometrics::prometheus_exporter;
pub mod guard;
pub mod library;
pub mod model;
pub mod repository;
pub mod service;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

#[get("/metrics")]
pub fn metrics() -> String {
    prometheus_exporter::encode_to_string().unwrap()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    prometheus_exporter::init();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .unwrap();

    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build()
        .manage(pool)
        .attach(cors)
        .mount("/", routes![metrics])
        .attach(route_stage())
        .launch()
        .await
        .unwrap();
}
