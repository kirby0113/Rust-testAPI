#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: Option<ObjectId>,
    pub title: String;
    pub category: String;
    pub detail: String;
    pub limit: String;
    pub created_at: Date;
    pub checked: Boolean;
    pub isFavorite: Boolean;
}