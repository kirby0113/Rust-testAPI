use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub _id: Option<ObjectId>,
    pub name: String,
}