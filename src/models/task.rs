use chrono::{Utc,Date};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Clone)]
pub struct User {
    pub _id: Option<ObjectId>,
    pub title: String,
    pub category: String,
    pub detail: String,
    pub limit: String,
    pub created_at: Date<Utc>,
    pub checked: bool,
    pub isFavorite: bool,
}