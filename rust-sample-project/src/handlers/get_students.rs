use actix_web::{web, HttpResponse, Responder, HttpRequest};
use mongodb::bson::doc;
use futures::stream::StreamExt;
use crate::db::marks_db::get_marks_collection;
use crate::models::Marks;
use std::collections::HashMap;

pub async fn get_students(req: HttpRequest, query: web::Query<HashMap<String, String>>) -> impl Responder {
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
    // let grade_param = query.get("grade").unwrap_or(&"pass".to_string()).to_lowercase();

    // Fetch data from the MongoDB collection
    let collection = get_marks_collection().await;

    let mut cursor = collection.find(None, None).await.unwrap();

    let mut results: HashMap<String, String> = HashMap::new();
    while let Some(document) = cursor.next().await {
        match document {
            Ok(marks) => {
                // Calculate average marks
                let average = (marks.subject1 + marks.subject2) as f32 / 2.0;
                let student_grade = if average > 33.0 { "pass" } else { "fail" };

                // Add the student to results if their grade matches the query parameter
                
                    results.insert(marks.username.clone(), student_grade.to_string());
               
            }
            Err(err) => {
                eprintln!("Error retrieving document: {:?}", err);
                return HttpResponse::InternalServerError().body("Failed to retrieve student records");
            }
        }
    }

    HttpResponse::Ok().json(results) // Return the results as JSON
}
