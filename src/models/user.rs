use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String,
}