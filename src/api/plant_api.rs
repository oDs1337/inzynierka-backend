use crate::{models::plant_model::Plant,models::plant_model::Data, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/create_plant", data = "<new_plant>")]
pub fn create_plant(
    db: &State<MongoRepo>,
    new_plant: Json<Plant>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Plant {
        id: None,
        name: new_plant.name.to_owned(),
        data: new_plant.data.iter().cloned().collect::<Vec<Data>>(),
    };
    let plant_detail = db.create_plant(data);
    match plant_detail {
        Ok(plant) => Ok(Json(plant)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/plants")]
pub fn get_all_plants(db: &State<MongoRepo>) -> Result<Json<Vec<Plant>>, Status> {
    let plants = db.get_all_plants();
    match plants {
        Ok(plants) => Ok(Json(plants)),
        Err(_) => Err(Status::InternalServerError),
    }
}