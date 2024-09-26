use actix_web::{web, HttpResponse, Responder};
use mongodb::bson::doc; // Uncomment if you need to create BSON documents
use crate::db::marks_db::get_marks_collection;
use crate::models::Marks;

// Add marks operation
pub async fn add_marks(marks: web::Json<Marks>) -> impl Responder {
    // Check if the user is a teacher
    if marks.user_type != "teacher" {
        return HttpResponse::Unauthorized().body("Only teachers can add marks");
    }

    let collection = get_marks_collection().await;
    let new_marks = marks.into_inner();

    // Check if marks already exist for the given student_id
    let existing_marks = collection
        .find_one(doc! { "username": &new_marks.username }, None)
        .await;

    match existing_marks {
        Ok(Some(_)) => {
            // Marks already exist for this student
            return HttpResponse::Conflict().body("Marks for this student already exist");
        }
        Ok(None) => {
            // Marks do not exist, proceed to insert
            match collection.insert_one(new_marks, None).await {
                Ok(inserted) => HttpResponse::Created().json(inserted.inserted_id),
                Err(err) => {
                    eprintln!("Database error: {:?}", err);
                    HttpResponse::InternalServerError().body("Failed to add marks")
                }
            }
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to query marks")
        }
    }
}
