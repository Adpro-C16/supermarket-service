use rocket::fairing::AdHoc;

pub mod supermarket;

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
            ],
        )
    });
}
