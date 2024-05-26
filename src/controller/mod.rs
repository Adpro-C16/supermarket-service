use rocket::fairing::AdHoc;

pub mod supermarket;
pub mod supermarket_test;

pub fn route_stage() -> AdHoc {
    return AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        rocket.mount(
            "/supermarkets",
            routes![
                supermarket::find_all,
                supermarket::find_by_id,
                supermarket::create,
                supermarket::update,
                supermarket::delete,
                supermarket::topup_supermarket_balance,
                supermarket::list_all_users
            ],
        )
    });
}
