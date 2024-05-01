use controller::route_stage;
use rocket::tokio;
use std::env;

#[macro_use]
extern crate rocket;
pub mod controller;
pub mod services {
    tonic::include_proto!("services");
}

pub mod guard;
pub mod library;
pub mod model;
pub mod repository;
pub mod service;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .unwrap();

    rocket::build()
        .manage(pool)
        .attach(route_stage())
        .launch()
        .await
        .unwrap();
}
