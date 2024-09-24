use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use crate::db::user_db::get_db_collection;
use crate::models::User;
use crate::handlers::email::send_confirmation_email;

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
