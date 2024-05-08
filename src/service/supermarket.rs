use crate::{
    library::{error_response::error_response, heymart_result::Result},
    model::supermarket::{CreateSupermarketDto, Supermarket, UpdateSupermarketDto},
    repository::supermarket::SupermarketRepository,
};
use rocket::http::Status;
use sqlx::PgPool;
pub struct SupermarketService {}

impl SupermarketService {
    pub async fn get_all_supermarkets(db_pool: PgPool) -> Result<Vec<Supermarket>> {
        match SupermarketRepository::find_all(db_pool).await {
            Some(supermarkets) => Ok(supermarkets),
            None => Err(error_response(
                Status::InternalServerError,
                "Failed to get all supermarkets",
            )),
        }
    }

    pub async fn find_by_id(db_pool: PgPool, id: i64) -> Result<Supermarket> {
        match SupermarketRepository::find_by_id(db_pool, id).await {
            Some(supermarket) => Ok(supermarket),
            None => Err(error_response(Status::NotFound, "Supermarket not found")),
        }
    }

    pub async fn create(db_pool: PgPool, supermarket: CreateSupermarketDto) -> Result<Supermarket> {
        if supermarket.name.is_empty() {
            return Err(error_response(Status::BadRequest, "Name cannot be empty"));
        }

        if supermarket.manager_id < 1 {
            return Err(error_response(
                Status::BadRequest,
                "Manager ID must be greater than 0",
            ));
        }

        match SupermarketRepository::create(db_pool, supermarket).await {
            Some(supermarket) => Ok(supermarket),
            None => Err(error_response(
                Status::InternalServerError,
                "Failed to create supermarket",
            )),
        }
    }

    pub async fn update(
        db_pool: PgPool,
        id: i64,
        supermarket: UpdateSupermarketDto,
    ) -> Result<Supermarket> {
        match supermarket.name {
            Some(ref name) => {
                if name.is_empty() {
                    return Err(error_response(Status::BadRequest, "Name cannot be empty"));
                }
            }
            None => {}
        }

        match supermarket.clone().balance {
            Some(balance) => {
                if balance < 0 {
                    return Err(error_response(
                        Status::BadRequest,
                        "Balance cannot be negative",
                    ));
                }
            }
            None => {}
        }

        match SupermarketRepository::find_by_id(db_pool.clone(), id).await {
            Some(_) => match SupermarketRepository::update(db_pool, id, supermarket).await {
                Some(supermarket) => Ok(supermarket),
                None => Err(error_response(
                    Status::InternalServerError,
                    "Failed to update supermarket",
                )),
            },
            None => {
                return Err(error_response(Status::NotFound, "Supermarket not found"));
            }
        }
    }

    pub async fn delete(db_pool: PgPool, id: i64) -> Result<Supermarket> {
        match SupermarketRepository::find_by_id(db_pool.clone(), id).await {
            Some(supermarket) => match SupermarketRepository::delete(db_pool, id).await {
                Some(_) => Ok(supermarket),
                None => Err(error_response(
                    Status::InternalServerError,
                    "Failed to delete supermarket",
                )),
            },
            None => Err(error_response(Status::NotFound, "Supermarket not found")),
        }
    }
}
