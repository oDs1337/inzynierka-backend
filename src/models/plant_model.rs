use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Plant {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub data: Vec<Data>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Data {
    pub timestamp: i64,
    pub value: i64,
}
