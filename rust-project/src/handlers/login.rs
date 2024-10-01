use actix_web::{web, HttpResponse, Responder};
use bcrypt::verify;
use mongodb::bson::doc;
use crate::db::user_db::get_db_collection;
use crate::models::UserLogin;
use serde::{Deserialize, Serialize}; // Use serde instead of serde_json
use serde_json::json;
use jsonwebtoken::{encode, Header, EncodingKey};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

// JWT secret loaded from environment variable
fn get_jwt_secret() -> Vec<u8> {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set").into_bytes()
}

// Define your claims struct
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// Login operation: Authenticate the user
pub async fn login_user(user: web::Json<UserLogin>) -> impl Responder {
    let collection = get_db_collection().await;
    let filter = doc! { "username": &user.username };
    let result = collection.find_one(filter, None).await.unwrap();

    match result {
        Some(found_user) => {
            if verify(&user.password, &found_user.password).unwrap() {
                // Generate JWT token
                let claims = Claims {
                    sub: found_user.username.clone(),
                    exp: (SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap().as_secs() + 3600) as usize, // Token valid for 1 hour
                };

                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(&get_jwt_secret())).unwrap();

                HttpResponse::Ok().json(json!({
                    "message": "Login successful",
                    "token": token
                }))
            } else {
                HttpResponse::Unauthorized().body("Invalid password")
            }
        }
        None => HttpResponse::NotFound().body("User not found"),
    }
}
