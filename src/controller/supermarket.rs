use crate::model::supermarket::{CreateSupermarketDto, UpdateSupermarketDto};
use crate::repository::supermarket::UserData;
use crate::service::supermarket::SupermarketService;
use crate::{
    guard::auth_guard::{AuthGuard, Role::Admin},
    library::error_response::error_response,
};
use crate::{library::heymart_result::Result, model::supermarket::Supermarket};
use autometrics::autometrics;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, http::Status, post, put, State};
use rocket::{response::status::Created, serde::json::Json};
use sqlx::PgPool;

#[get("/")]
#[autometrics]
pub async fn find_all(db_pool: &State<PgPool>) -> Result<Json<Vec<Supermarket>>> {
    return match SupermarketService::get_all_supermarkets(db_pool.inner().clone()).await {
        Ok(supermarkets) => Ok(Json::from(supermarkets)),
        Err(e) => Err(e),
    };
}

#[get("/<id>")]
#[autometrics]
pub async fn find_by_id(db_pool: &State<PgPool>, id: i64) -> Result<Json<Supermarket>> {
    return match SupermarketService::find_by_id(db_pool.inner().clone(), id).await {
        Ok(supermarket) => Ok(Json::from(supermarket)),
        Err(e) => Err(e),
    };
}

#[post("/", format = "json", data = "<supermarket>")]
#[autometrics]
pub async fn create(
    auth_ctx: AuthGuard,
    db_pool: &State<PgPool>,
    supermarket: Json<CreateSupermarketDto>,
) -> Result<Created<Json<Supermarket>>> {
    if auth_ctx.claims.role != Admin {
        return Err(error_response(
            Status::Forbidden,
            "You are not authorized to perform this action.",
        ));
    }
    return match SupermarketService::create(db_pool.inner().clone(), supermarket.into_inner()).await
    {
        Ok(supermarket) => Ok(Created::new("/").body(Json(supermarket))),
        Err(e) => Err(e),
    };
}

#[put("/<id>", format = "json", data = "<supermarket>")]
#[autometrics]
pub async fn update(
    auth_ctx: AuthGuard,
    db_pool: &State<PgPool>,
    id: i64,
    supermarket: Json<UpdateSupermarketDto>,
) -> Result<Json<Supermarket>> {
    if auth_ctx.claims.role != Admin {
        return Err(error_response(
            Status::Forbidden,
            "You are not authorized to perform this action.",
        ));
    }
    return match SupermarketService::update(db_pool.inner().clone(), id, supermarket.into_inner())
        .await
    {
        Ok(supermarket) => Ok(Json::from(supermarket)),
        Err(e) => Err(e),
    };
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct TopUpSupermarketDto {
    amount: i32,
}

#[post("/topup/<id>", format = "json", data = "<topup>")]
#[autometrics]
pub async fn topup_supermarket_balance(
    db_pool: &State<PgPool>,
    id: i64,
    topup: Json<TopUpSupermarketDto>,
) -> Result<Json<Supermarket>> {
    let supermarket = SupermarketService::find_by_id(db_pool.inner().clone(), id).await;
    if supermarket.is_err() {
        return Err(error_response(Status::NotFound, "Supermarket not found"));
    }
    let mut supermarket = supermarket.unwrap();
    supermarket.balance += topup.amount;
    return match SupermarketService::update(
        db_pool.inner().clone(),
        id,
        UpdateSupermarketDto {
            balance: Some(supermarket.balance),
            manager_id: None,
            name: None,
        },
    )
    .await
    {
        Ok(supermarket) => Ok(Json::from(supermarket)),
        Err(e) => Err(e),
    };
}

#[delete("/<id>")]
#[autometrics]
pub async fn delete(auth_ctx: AuthGuard, db_pool: &State<PgPool>, id: i64) -> Result<()> {
    if auth_ctx.claims.role != Admin {
        return Err(error_response(
            Status::Forbidden,
            "You are not authorized to perform this action.",
        ));
    }
    return match SupermarketService::delete(db_pool.inner().clone(), id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}

#[get("/users")]
#[autometrics]
pub async fn list_all_users(
    auth_ctx: AuthGuard,
    db_pool: &State<PgPool>,
) -> Result<Json<Vec<UserData>>> {
    if auth_ctx.claims.role != Admin {
        return Err(error_response(
            Status::Forbidden,
            "You are not authorized to perform this action.",
        ));
    }
    return match SupermarketService::get_all_users(db_pool.inner().clone()).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err(e),
    };
}

#[get("/manager/<manager_id>")]
#[autometrics]
pub async fn find_by_manager_id(
    db_pool: &State<PgPool>,
    manager_id: i32,
) -> Result<Json<Vec<Supermarket>>> {
    return match SupermarketService::find_by_manager_id(db_pool.inner().clone(), manager_id).await {
        Ok(supermarkets) => Ok(Json::from(supermarkets)),
        Err(e) => Err(e),
    };
}
