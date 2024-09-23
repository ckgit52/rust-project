use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use mongodb::{bson::{doc, oid::ObjectId}, Client, Collection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// User struct ko define karte hain. Yeh MongoDB document ke saath match karega.
#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    username: String,
    email: String,
    password: String,
    user_type: String, // User type: student, teacher, admin
}

// MongoDB se connection establish karne ke liye setup function banate hain.
async fn get_db_collection() -> Collection<User> {
    let client = Client::with_uri_str("mongodb+srv://chandanrust:chandanrust@rust-project.dhmtb.mongodb.net/").await.unwrap(); // Localhost connection
    let db = client.database("registration");
    db.collection::<User>("users")
}

// Create operation: Naya user add karne ka function
async fn create_user(user: web::Json<User>) -> impl Responder {
    let collection = get_db_collection().await;
    let  new_user = user.into_inner();
    
    let insert_result = collection.insert_one(new_user, None).await;
    match insert_result {
        Ok(inserted) => HttpResponse::Created().json(inserted.inserted_id),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
    }
}



// Actix Web ko setup karte hain aur endpoints define karte hain
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/register", web::post().to(create_user))           // Create user
            
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
