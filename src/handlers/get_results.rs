use actix_web::{web, HttpRequest, HttpResponse, Responder};
use mongodb::bson::{doc, Document};
use futures::stream::StreamExt;
use crate::db::marks_db::get_marks_collection;

pub async fn get_results(req: HttpRequest) -> impl Responder {
    // Check if the user is a student
    let user_role = req.headers().get("Student").and_then(|h| h.to_str().ok());
    if let Some(role) = user_role {
        if role != "student" {
            return HttpResponse::Forbidden().body("Only students can access this endpoint");
        }
    } else {
        return HttpResponse::Forbidden().body("Student  not found in header");
    }

    // Fetch data from the MongoDB collection
    let collection = get_marks_collection().await;
    let mut cursor = collection.find(None, None).await.unwrap();

    let mut results: Vec<Document> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                // Extract necessary fields
                let username = document.username;
                let subject1 = document.subject1;
                let subject2 = document.subject2;

                // Calculate total and average
                let total = subject1 + subject2;
                let average = total as f32 / 2.0;

                // Determine grade
                let grade = if average > 33.0 { "Pass" } else { "Fail" };

                // Create a JSON document for the result
                let student_result = doc! {
                    "username": username,
                    "subject1": subject1,
                    "subject2": subject2,
                    "total": total,
                    "average": average,
                    "grade": grade
                };

                results.push(student_result);
            }
            Err(err) => {
                eprintln!("Error fetching student data: {:?}", err);
                return HttpResponse::InternalServerError().body("Failed to fetch student data");
            }
        }
    }

    HttpResponse::Ok().json(results) // Return the results as JSON
}
