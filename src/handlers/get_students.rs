use actix_web::{web, HttpResponse, Responder, HttpRequest};
use mongodb::bson::{doc};
use futures::stream::StreamExt; // Import for stream extensions
use futures_util::TryStreamExt; // Import for try_next
use crate::db::marks_db::get_marks_collection;
use crate::models::Marks; // Import Marks struct

pub async fn get_students(req: HttpRequest, query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    // Check if the user is a teacher
    let user_role = req.headers().get("Role").and_then(|h| h.to_str().ok());
    if let Some(role) = user_role {
        if role != "teacher" {
            return HttpResponse::Forbidden().body("Only teachers can access this endpoint");
        }
    } else {
        return HttpResponse::Forbidden().body("Role not found");
    }

    // Get the grade parameter from the query string
    let grade_param = query.get("grade").unwrap_or(&"pass".to_string()).to_lowercase();

    // Fetch data from the MongoDB collection
    let collection = get_marks_collection().await;

    let mut cursor = collection.find(None, None).await.unwrap();

    let mut results: Vec<Marks> = Vec::new();
    while let Some(result) = cursor.try_next().await.unwrap() {
        // Assuming `result` is of type `Marks`
        let marks: Marks = result;

        // Calculate average marks
        let average = (marks.subject1 + marks.subject2) as f32 / 2.0;
        let student_grade = if average > 33.0 { "pass" } else { "fail" };

        // Check if the student's grade matches the query parameter
        if student_grade == grade_param {
            results.push(marks);
        }
    }

    HttpResponse::Ok().json(results) // Return the results as JSON
}
