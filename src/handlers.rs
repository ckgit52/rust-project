use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use mongodb::{bson::doc, Collection};
use serde_json::json;
use std::env;

use crate::db::{get_db_collection, get_marks_collection};
use crate::models::{User, UserLogin, Marks, PasswordChangeRequest};

// Email confirmation function
async fn send_confirmation_email(email: &str) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_username = env::var("SMTP_USERNAME")?;
    let smtp_password = env::var("SMTP_PASSWORD")?;
    let smtp_host = env::var("SMTP_HOST")?;
    let smtp_port = env::var("SMTP_PORT")?.parse::<u16>()?;

    let creds = Credentials::new(smtp_username.clone(), smtp_password);
    let mailer = SmtpTransport::relay(&smtp_host)?
        .port(smtp_port)
        .credentials(creds)
        .build();

    let email = Message::builder()
        .from(smtp_username.parse()?)
        .to(email.parse()?)
        .subject("Registration Confirmation")
        .body("Thank you for registering!".to_string())
        .unwrap();

    mailer.send(&email)?;
    Ok(())
}

// Create operation: Add a new user
pub async fn create_user(user: web::Json<User>) -> impl Responder {
    let collection = get_db_collection().await;

    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();
    let mut new_user = user.into_inner();
    new_user.password = hashed_password;

    match collection.insert_one(new_user, None).await {
        Ok(inserted) => {
            if let Err(email_error) = send_confirmation_email(&inserted.inserted_id.to_string()).await {
                eprintln!("Email sending error: {:?}", email_error);
                return HttpResponse::InternalServerError().body("Failed to send confirmation email");
            }
            HttpResponse::Created().json(inserted.inserted_id)
        }
        Err(db_error) => {
            eprintln!("Database error: {:?}", db_error);
            HttpResponse::InternalServerError().body("Failed to create user in the database")
        }
    }
}

// Login operation: Authenticate the user
pub async fn login_user(user: web::Json<UserLogin>) -> impl Responder {
    let collection = get_db_collection().await;
    let filter = doc! { "username": &user.username };
    let result = collection.find_one(filter, None).await.unwrap();

    match result {
        Some(found_user) => {
            if verify(&user.password, &found_user.password).unwrap() {
                HttpResponse::Ok().json(json!({"message": "Login successful"}))
            } else {
                HttpResponse::Unauthorized().body("Invalid password")
            }
        }
        None => HttpResponse::NotFound().body("User not found"),
    }
}

// Forgot password operation: Update user password
pub async fn forgot_password(request: web::Json<PasswordChangeRequest>) -> impl Responder {
    let collection = get_db_collection().await;
    let filter = doc! { "username": &request.username };
    let result = collection.find_one(filter.clone(), None).await.unwrap();

    match result {
        Some(mut user) => {
            if verify(&request.old_password, &user.password).unwrap() {
                if request.old_password == request.new_password {
                    return HttpResponse::BadRequest().body("New password cannot be the same as the old password.");
                }

                let hashed_new_password = hash(&request.new_password, DEFAULT_COST).unwrap();
                user.password = hashed_new_password;

                let update_doc = doc! { "$set": { "password": &user.password } };
                let update_result = collection.update_one(filter, update_doc, None).await;

                match update_result {
                    Ok(_) => HttpResponse::Ok().body("Password updated successfully."),
                    Err(_) => HttpResponse::InternalServerError().body("Failed to update password."),
                }
            } else {
                HttpResponse::Unauthorized().body("Old password is incorrect.")
            }
        }
        None => HttpResponse::NotFound().body("User not found"),
    }
}

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
