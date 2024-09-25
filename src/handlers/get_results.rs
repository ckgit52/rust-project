use actix_web::{web, HttpResponse, Responder};
use mongodb::bson::doc;
use crate::db::marks_db::get_marks_collection; // Adjust this path as needed
use crate::models::results::{Result, StudentResults}; // Adjust the path and ensure `Result` is defined in your models
use futures_util::TryStreamExt; // Correct import for `try_next`

pub async fn get_results(student_id: web::Path<String>) -> impl Responder {
    let collection = get_marks_collection().await;

    let filter = doc! { "student_id": student_id.as_str() }; // Convert Path to str

    // Use `try_next` with the cursor
    let mut cursor = match collection.find(filter, None).await {
        Ok(cursor) => cursor,
        Err(_) => return HttpResponse::InternalServerError().json("Error fetching data"),
    };

    let mut results = Vec::new();
    let mut total = 0;

    while let Some(doc) = cursor.try_next().await.unwrap_or(None) {
        let subject = doc.get_str("subject").unwrap_or("Unknown").to_string();
        let score = doc.get_i32("score").unwrap_or(0);

        results.push(Result { subject, score });
        total += score;
    }

    // Calculate grade based on total
    let grade = match total {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };

    let response = StudentResults {
        student_id: student_id.into_inner(),
        results,
        total,
        grade: grade.to_string(),
    };

    HttpResponse::Ok().json(response)
}
