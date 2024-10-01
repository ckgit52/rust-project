use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    pub subject: String,
    pub score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentResults {
    pub student_id: String,
    pub results: Vec<Result>,
    pub total: i32,
    pub grade: String,
}


