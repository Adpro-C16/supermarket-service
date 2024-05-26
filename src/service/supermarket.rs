use crate::{
    library::{error_response::error_response, heymart_result::Result},
    model::supermarket::{CreateSupermarketDto, Supermarket, UpdateSupermarketDto},
    repository::supermarket::{SupermarketRepository, UserData},
};
use rocket::http::Status;
use rocket::log;
use sqlx::PgPool;
pub struct SupermarketService {}

impl SupermarketService {
    pub async fn get_all_supermarkets(db_pool: PgPool) -> Result<Vec<Supermarket>> {
        match SupermarketRepository::find_all(db_pool).await {
            Some(supermarkets) => {
                log::info_!("retrieved {} supermarkets", supermarkets.len());
                Ok(supermarkets)
            }
            None => {
                log::error_!("Failed to get all supermarkets");
                Err(error_response(
                    Status::InternalServerError,
                    "Failed to get all supermarkets",
                ))
            }
        }
    }

    pub async fn find_by_id(db_pool: PgPool, id: i64) -> Result<Supermarket> {
        match SupermarketRepository::find_by_id(db_pool, id).await {
            Some(supermarket) => {
                log::info_!(
                    "retrieved supermarket | name={}, balace={}, manager_id={}",
                    supermarket.name,
                    supermarket.balance,
                    supermarket.manager_id
                );
                Ok(supermarket)
            }
            None => {
                log::warn_!("Supermarket not found");
                Err(error_response(Status::NotFound, "Supermarket not found"))
            }
        }
    }

    pub async fn create(db_pool: PgPool, supermarket: CreateSupermarketDto) -> Result<Supermarket> {
        log::info_!(
            "creating supermarket | name={}, manager_id={}",
            supermarket.name,
            supermarket.manager_id
        );
        if supermarket.name.is_empty() {
            log::warn_!("failed to create supermarket | name cannot be empty");
            return Err(error_response(Status::BadRequest, "Name cannot be empty"));
        }

        if supermarket.manager_id < 1 {
            log::warn_!("failed to create supermarket | manager_id must be greater than 0");
            return Err(error_response(
                Status::BadRequest,
                "Manager ID must be greater than 0",
            ));
        }

        match SupermarketRepository::create(db_pool, supermarket).await {
            Some(supermarket) => {
                log::info_!(
                    "created supermarket | name={}, balance={}, manager_id={}",
                    supermarket.name,
                    supermarket.balance,
                    supermarket.manager_id
                );
                Ok(supermarket)
            }
            None => {
                log::error_!("Failed to create supermarket");
                Err(error_response(
                    Status::InternalServerError,
                    "Failed to create supermarket",
                ))
            }
        }
    }

    pub async fn update(
        db_pool: PgPool,
        id: i64,
        supermarket: UpdateSupermarketDto,
    ) -> Result<Supermarket> {
        log::info_!(
            "updating supermarket | id={}, name={}, balance={}",
            id,
            supermarket.name.clone().unwrap_or("N/A".to_string()),
            supermarket.balance.clone().unwrap_or(0)
        );
        match supermarket.name {
            Some(ref name) => {
                if name.is_empty() {
                    log::warn_!("failed to update supermarket | name cannot be empty");
                    return Err(error_response(Status::BadRequest, "Name cannot be empty"));
                }
            }
            None => {}
        }

        match supermarket.clone().balance {
            Some(balance) => {
                if balance < 0 {
                    log::warn_!("failed to update supermarket | balance cannot be negative");
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
                Some(supermarket) => {
                    log::info_!(
                        "updated supermarket | name={}, balance={}",
                        supermarket.name,
                        supermarket.balance
                    );
                    Ok(supermarket)
                }
                None => {
                    log::error_!("Failed to update supermarket");
                    Err(error_response(
                        Status::InternalServerError,
                        "Failed to update supermarket",
                    ))
                }
            },
            None => {
                log::warn_!("Supermarket not found");
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

    pub async fn get_all_users(db_pool: PgPool) -> Result<Vec<UserData>> {
        match SupermarketRepository::list_all_users(db_pool).await {
            Some(users) => {
                log::info_!("retrieved {} users in supermarket", users.len());
                Ok(users)
            }
            None => {
                log::error_!("Failed to get all users in supermarket");
                Err(error_response(
                    Status::InternalServerError,
                    "Failed to get all users in supermarket",
                ))
            }
        }
    }

    pub async fn find_by_manager_id(db_pool: PgPool, manager_id: i32) -> Result<Vec<Supermarket>> {
        match SupermarketRepository::find_by_manager_id(db_pool, manager_id).await {
            Some(supermarket) => {
                log::info_!("retrieved supermarket with manager_id={}", manager_id);
                Ok(supermarket)
            }
            None => {
                log::warn_!("Supermarket not found");
                Err(error_response(Status::NotFound, "Supermarket not found"))
            }
        }
    }
}
