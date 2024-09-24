use actix_web::{web, HttpResponse, Responder};
// use mongodb::bson::doc;
use crate::db::marks_db::get_marks_collection;
use crate::models::Marks;

// Add marks operation
pub async fn add_marks(marks: web::Json<Marks>) -> impl Responder {
    if marks.user_type != "teacher" {
        return HttpResponse::Unauthorized().body("Only teachers can add marks");
    }

    let collection = get_marks_collection().await;
    let new_marks = marks.into_inner();

    match collection.insert_one(new_marks, None).await {
        Ok(inserted) => HttpResponse::Created().json(inserted.inserted_id),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to add marks")
        }
    }
}
