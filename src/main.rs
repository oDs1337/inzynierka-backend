mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::plant_api::{create_plant, get_all_plants};
use repository::mongodb_repo::MongoRepo;
use rocket_cors::{AllowedOrigins};

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::all();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        ..Default::default()
    };

    let db = MongoRepo::init();
    rocket::build()
        .attach(cors.to_cors().unwrap())
        .manage(db)
        .mount("/", routes![create_plant])
        .mount("/", routes![get_all_plants])
}