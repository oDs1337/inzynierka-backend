mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::plant_api::{create_plant, get_all_plants};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build().manage(db)
        .mount("/", routes![create_plant])
        .mount("/", routes![get_all_plants])
}