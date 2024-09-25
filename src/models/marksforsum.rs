use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Marks {
    pub student_id: String,
    pub subject: String,
    pub marks: Vec<i32>, // Assuming marks is stored as an array of integers
    pub grade: Option<String>,
}
