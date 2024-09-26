use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use mongodb::bson::doc;
use crate::db::user_db::get_db_collection;
use crate::models::PasswordChangeRequest;

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
