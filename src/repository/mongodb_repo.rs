use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error},
    results::{ InsertOneResult},
    sync::{Client, Collection},
};
use crate::models::plant_model::Plant;

pub struct MongoRepo {
    col: Collection<Plant>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => "Error loading env variable".to_string(),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("plantly_db");
        let col: Collection<Plant> = db.collection("plantly");
        MongoRepo { col }
    }

    pub fn create_plant(&self, new_plant: Plant) -> Result<InsertOneResult, Error> {
        let new_doc = Plant {
            id: None,
            name: new_plant.name,
            data: new_plant.data,
        };
        let plant = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating plant");
        Ok(plant)
    }

    pub fn get_all_plants(&self) -> Result<Vec<Plant>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of plants");
        let plants = cursors.map(|doc| doc.unwrap()).collect();
        Ok(plants)
    }
}