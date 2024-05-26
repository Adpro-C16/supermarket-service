use crate::library::heymart_result::Result;
use crate::{
    guard::auth_guard::AuthGuard,
    library::error_response::error_response,
    repository::product::{CreateProductDto, Product, UpdateProductDto},
    service::product::ProductService,
};
use autometrics::autometrics;
use rocket::serde::json::Json;
use rocket::{get, http::Status, post, put, State};
use sqlx::PgPool;

#[get("/")]
#[autometrics]
pub async fn find_all(db_pool: &State<PgPool>) -> Result<Json<Vec<Product>>> {
    return match ProductService::find_all(db_pool.inner().clone()).await {
        Some(products) => Ok(Json(products)),
        None => Err(error_response(Status::NotFound, "No products found.")),
    };
}

#[get("/<id>")]
#[autometrics]
pub async fn find_by_id(db_pool: &State<PgPool>, id: i32) -> Result<Json<Product>> {
    return match ProductService::find_by_id(db_pool.inner().clone(), id).await {
        Some(product) => Ok(Json::from(product)),
        None => Err(error_response(Status::NotFound, "Product not found.")),
    };
}

#[post("/", format = "json", data = "<product>")]
#[autometrics]
pub async fn create(
    _auth_ctx: AuthGuard,
    db_pool: &State<PgPool>,
    product: Json<CreateProductDto>,
) -> Result<Json<Product>> {
    return match ProductService::create(db_pool.inner().clone(), product.into_inner()).await {
        Some(product) => Ok(Json::from(product)),
        None => Err(error_response(
            Status::InternalServerError,
            "Failed to create product.",
        )),
    };
}

#[put("/<id>", format = "json", data = "<product>")]
#[autometrics]
pub async fn update(
    _auth_ctx: AuthGuard,
    db_pool: &State<PgPool>,
    id: i32,
    product: Json<UpdateProductDto>,
) -> Result<Json<Product>> {
    return match ProductService::update(db_pool.inner().clone(), id, product.into_inner()).await {
        Some(product) => Ok(Json::from(product)),
        None => Err(error_response(
            Status::InternalServerError,
            "Failed to update product.",
        )),
    };
}

#[delete("/<id>")]
#[autometrics]
pub async fn delete(
    _auth_ctx: AuthGuard,
    db_pool: &State<PgPool>,
    id: i32,
) -> Result<Json<Product>> {
    return match ProductService::delete(db_pool.inner().clone(), id).await {
        Some(product) => Ok(Json::from(product)),
        None => Err(error_response(
            Status::InternalServerError,
            "Failed to delete product.",
        )),
    };
}
