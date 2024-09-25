// Import necessary crates
use actix_web::{web, HttpResponse, Responder, HttpRequest};
use mongodb::bson::{doc, oid::ObjectId};
use futures::stream::StreamExt; // Import for stream extensions
use futures_util::TryStreamExt; // Import for try_next
use crate::db::marks_db::get_marks_collection;
use crate::models::Marks; // Import Marks struct
use serde::{Serialize, Deserialize};

// Define Mark struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mark {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub student_id: String,
    pub marks: Vec<i32>, // Assuming marks is a list of integers for different subjects
    pub total: Option<i32>, // Optional total field
    pub grade: Option<String>, // Optional grade field
}

// Implement methods for Mark
impl Mark {
    pub fn calculate_total_and_grade(&mut self) {
        self.total = Some(self.marks.iter().sum());
        self.grade = match self.total {
            Some(total) if total >= 90 => Some("A".to_string()),
            Some(total) if total >= 75 => Some("B".to_string()),
            Some(total) if total >= 60 => Some("C".to_string()),
            Some(total) if total >= 45 => Some("D".to_string()),
            _ => Some("F".to_string()),
        };
    }
}

pub async fn get_results(req: HttpRequest) -> impl Responder {
    // Check if the user is a student
    let user_role = req.headers().get("Role").and_then(|h| h.to_str().ok());
    if let Some(role) = user_role {
        if role != "student" {
            return HttpResponse::Forbidden().body("Only students can access this endpoint");
        }
    } else {
        return HttpResponse::Forbidden().body("Role not found");
    }

    // Fetch data from the MongoDB collection
    let collection = get_marks_collection().await;

    // Get student_id from request headers
    let student_id = req.headers().get("student_id").and_then(|h| h.to_str().ok()).unwrap_or_default();

    // Create filter for student_id
    let filter = doc! { "student_id": student_id };
    let mut cursor = collection.find(filter, None).await.unwrap();

    let mut results: Vec<Mark> = Vec::new();
    while let Some(result) = cursor.try_next().await.unwrap() {
        // Assuming `result` is of type `Marks`
        let marks: Marks = result;

        // Map fields from Marks to Mark
        let mut mark = Mark {
            id: marks.id, // If id exists in Marks
            student_id: marks.student_id,
            // Assuming you have subjects stored in Marks and want to calculate marks
            marks: vec![marks.subject1, marks.subject2], // Adjust this if there are more subjects
            total: None, // Total will be calculated
            grade: None, // Grade will be calculated
        };

        // Calculate total and grade
        mark.calculate_total_and_grade();
        results.push(mark);
    }

    HttpResponse::Ok().json(results) // Return the results as JSON
}
