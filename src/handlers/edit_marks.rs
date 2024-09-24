use actix_web::{web, HttpResponse, Responder};
use mongodb::bson::{doc, Bson};
use crate::db::marks_db::get_marks_collection; // Adjust path as needed
use crate::handlers::email::send_notification_email; // Adjust path as needed
use crate::models::edit_marks_request::EditMarksRequest; // Use the new struct

// Edit marks operation
pub async fn edit_mark(mark_info: web::Json<EditMarksRequest>) -> impl Responder {
    let collection = get_marks_collection().await;

    let request = mark_info.into_inner();

    // Update the marks in the database
    let filter = doc! {
        "student_id": &request.student_id,
        "subject": &request.subject,
    };

    let update = doc! {
        "$set": { "score": request.new_score },
    };

    match collection.update_one(filter, update, None).await {
        Ok(update_result) => {
            if update_result.modified_count == 1 {
                // Send notification email to the student
                let email_body = format!(
                    "Dear Student, your teacher {}, edited your marks in {} subject from {} to {}.",
                    request.teacher_name, request.subject, request.old_score, request.new_score
                );

                if let Err(email_error) = send_notification_email(&request.student_id, &email_body).await {
                    eprintln!("Email sending error: {:?}", email_error);
                    return HttpResponse::InternalServerError().body("Failed to send notification email");
                }

                HttpResponse::Ok().json({
                    let mut response = serde_json::json!({"message": "Marks updated successfully"});
                    response["modified_count"] = update_result.modified_count.into();
                    response
                })
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
