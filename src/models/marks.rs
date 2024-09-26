use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Marks {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub subject1: i32,
    pub subject2: i32,
    // pub teacher_name: String, // Teacher ka naam
    // pub user_type: String, // User type to check if the user is a teacher
}
