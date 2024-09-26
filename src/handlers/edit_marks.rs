use actix_web::{web, HttpResponse, Responder};
use mongodb::bson::doc;
use crate::db::marks_db::get_marks_collection;
use crate::db::user_db::get_db_collection; // Adjust based on your project structure
use crate::models::edited_mark::EditMarksRequest;
use crate::handlers::send_email::send_confirmation_email; // Ensure this path is correct

pub async fn edit_mark(mark_info: web::Json<EditMarksRequest>) -> impl Responder {
    // Check if the user is a teacher
    if mark_info.user_type != "teacher" {
        return HttpResponse::Unauthorized().body("Only teachers can edit marks");
    }

    let collection = get_marks_collection().await;
    let request = mark_info.into_inner();

    // Initialize a variable to keep track of changes
    let mut updated_fields = doc! {};
    
    // Check if subject1 is provided and update accordingly
    if let Some(score) = request.subject1 {
        updated_fields.insert("subject1", score);
    }
    
    // Check if subject2 is provided and update accordingly
    if let Some(score) = request.subject2 {
        updated_fields.insert("subject2", score);
    }

    // If no subjects were provided, return an error
    if updated_fields.is_empty() {
        return HttpResponse::BadRequest().body("No marks to update");
    }

    // Update the marks in the database using username instead of student_id
    let filter = doc! { "username": &request.username }; // Use username to identify the student
    let update = doc! { "$set": updated_fields };

    match collection.update_one(filter, update, None).await {
        Ok(update_result) => {
            if update_result.modified_count > 0 {
                // Fetch the student's email using username
                let student_collection = get_db_collection().await; // Ensure this function is implemented
                let student_filter = doc! { "username": &request.username }; // Use username to fetch the student
                
                match student_collection.find_one(student_filter, None).await {
                    Ok(Some(student)) => {
                        // Send confirmation email
                        if let Err(email_error) = send_confirmation_email(&student.email).await {
                            eprintln!("Email sending error: {:?}", email_error);
                            return HttpResponse::InternalServerError().body("Failed to send confirmation email");
                        }

                        HttpResponse::Ok().json({
                            let mut response = serde_json::json!({"message": "Marks updated successfully"});
                            response["modified_count"] = update_result.modified_count.into();
                            response["teacher_name"] = serde_json::Value::String(request.teacher_name.clone()); // Include teacher's name in response

                            response
                        })
                    }
                    Ok(None) => HttpResponse::NotFound().body("Student not found"),
                    Err(err) => {
                        eprintln!("Database error: {:?}", err);
                        HttpResponse::InternalServerError().body("Failed to retrieve student email")
                    }
                }
            } else {
                HttpResponse::NotFound().body("No matching record found to update")
            }
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to update marks in the database")
        }
    }
}
