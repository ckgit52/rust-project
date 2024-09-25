use actix_web::{web, HttpResponse, Responder, HttpRequest};
use mongodb::{Collection, bson::doc};

use crate::db::marks_db::get_marks_collection;

// Function to delete student marks by student_id
pub async fn delete_marks(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    // Extract student_id from the path
    let student_id = path.into_inner(); // Get the inner String from Path

    // Check if the user is a teacher (this could be replaced with your actual authorization logic)
    let user_role = req.headers().get("Role").and_then(|h| h.to_str().ok());

    if let Some(role) = user_role {
        if role != "teacher" {
            return HttpResponse::Forbidden().body("Only teachers can delete marks");
        }
    } else {
        return HttpResponse::Forbidden().body("Role not found");
    }

    // Get the MongoDB collection
    let collection = get_marks_collection().await;

    // Create the filter using the student_id
    let filter = doc! { "student_id": student_id }; // Use student_id directly

    // Perform the delete operation
    match collection.delete_one(filter, None).await {
        Ok(result) => {
            if result.deleted_count > 0 {
                HttpResponse::Ok().body("Marks deleted successfully")
            } else {
                HttpResponse::NotFound().body("No marks found for this student ID")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error deleting marks"),
    }
}
