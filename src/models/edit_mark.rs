use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EditMarksRequest {
    pub student_id: String,
    pub subject: String,
    pub new_score: u32,
    pub old_score: u32,
    pub teacher_name: String, // Name of the teacher making the edit
}
