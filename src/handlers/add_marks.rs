use actix_web::{web, HttpResponse, Responder,HttpRequest};
use mongodb::bson::doc; // Uncomment if you need to create BSON documents
use crate::db::marks_db::get_marks_collection;
use crate::models::Marks;

// Add marks operation
pub async fn add_marks(marks: web::Json<Marks>, req: HttpRequest) -> impl Responder {
    // Check if the user is a teacher
    // if marks.user_type != "teacher" {
    //     return HttpResponse::Unauthorized().body("Only teachers can add marks");
    // }
    let user_role = req.headers().get("Role").and_then(|h| h.to_str().ok());

    let is_student = req.headers().get("Student").and_then(|h| h.to_str().ok());

    if let Some(role) = user_role {
        if role != "teacher" {
            return HttpResponse::Forbidden().body("Only teachers can add marks");
        }
    } else {
        return HttpResponse::Forbidden().body("Role not found");
    }

    if let Some(role) = is_student {
        if role != "student" {
            return HttpResponse::Forbidden().body("only student marks can be added");
        }
    } else {
        return HttpResponse::Forbidden().body("student not found wrong body");
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
