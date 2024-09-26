use std::option;

// use serde::{Deserialize, Serialize};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EditMarksRequest {
    pub username: String,
    pub subject1: Option<i32>, // Optional score for subject 1
    pub subject2: Option<i32>, // Optional score for subject 2
    pub teacher_name: String,   // Name of the teacher making the edit
    pub user_type:String
}
