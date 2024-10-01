use actix_web::{web, HttpResponse, Responder, HttpRequest};
use mongodb::bson::{doc, oid::ObjectId};
use futures::stream::StreamExt; // Import for stream extensions
use futures_util::TryStreamExt; use crate::db::user_db::get_db_collection;
// Import for try_next
use crate::db::{marks_db::get_marks_collection}; // Adjust this import as needed
use crate::models::{Marks, User}; // Ensure `User` struct is defined for user data
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use std::env;
use mongodb::bson;


// Email sending function
async fn send_email(to_email: &str, subject: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_username = env::var("SMTP_USERNAME")?;
    let smtp_password = env::var("SMTP_PASSWORD")?;
    let smtp_host = "smtp.gmail.com"; // Adjust as needed
    let smtp_port = 587;

    let creds = Credentials::new(smtp_username.clone(), smtp_password);

    let mailer = SmtpTransport::relay(smtp_host)?
        .port(smtp_port)
        .credentials(creds)
        .build();

    let email = Message::builder()
        .from(smtp_username.parse()?)
        .to(to_email.parse()?)
        .subject(subject)
        .body(body.to_string())
        .unwrap();

    mailer.send(&email)?;
    Ok(())
}

// Handler for the /publish endpoint
pub async fn publish_results(req: HttpRequest) -> impl Responder {
    // Check if the user is an admin
    let user_role = req.headers().get("Role").and_then(|h| h.to_str().ok());
    if let Some(role) = user_role {
        if role != "admin" {
            return HttpResponse::Forbidden().body("Only admins can access this endpoint");
        }
    } else {
        return HttpResponse::Forbidden().body("Role not found");
    }

    // Fetch data from the MongoDB Marks collection
    let marks_collection = get_marks_collection().await;
    let mut cursor = marks_collection.find(None, None).await.unwrap();

    let users_collection = get_db_collection().await; // Fetch users collection

    while let Some(result) = cursor.try_next().await.unwrap() {
        let student_marks: Marks = result;

        // Fetch the student email from the users collection
        let filter = doc! { "student_id": &student_marks.student_id };
        if let Some(user_doc) = users_collection.find_one(filter, None).await.unwrap() {
            let user: User = bson::from_document(user_doc).unwrap();
            let student_email = user.email; // Make sure the User struct has an `email` field

            // Prepare email content
            let subject = "Your Exam Results";
            let body = format!(
                "Dear Student,\n\nYour results are as follows:\n\nSubject 1: {}\nSubject 2: {}\nTotal Marks: {}\nGrade: {}\n\nThank you.",
                student_marks.subject1,
                student_marks.subject2,
                student_marks.subject1 + student_marks.subject2,
                calculate_grade(student_marks.subject1, student_marks.subject2),
            );

            // Send email
            if let Err(e) = send_email(&student_email, subject, &body).await {
                eprintln!("Error sending email to {}: {}", student_email, e);
            }
        }
    }

    HttpResponse::Ok().body("Results have been sent to all students.")
}

// Helper function to calculate grade
fn calculate_grade(subject1: i32, subject2: i32) -> String {
    let average = (subject1 + subject2) / 2;
    match average {
        90..=100 => "A".to_string(),
        75..=89 => "B".to_string(),
        60..=74 => "C".to_string(),
        45..=59 => "D".to_string(),
        _ => "F".to_string(),
    }
}
