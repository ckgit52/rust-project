use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use mongodb::{bson::{doc, oid::ObjectId}, Client, Collection};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use dotenv::dotenv;
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;

// User struct ko define karte hain. Yeh MongoDB document ke saath match karega.
#[derive(Debug, Serialize, Deserialize,Clone)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    username: String,
    email: String,
    password: String,
    user_type: String, // User type: student, teacher, admin
}

#[derive(Debug, Serialize, Deserialize,Clone)]
struct Userlogin {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    username: String,
    
    password: String,
    
}

// MongoDB se connection establish karne ke liye setup function banate hain.
async fn get_db_collection() -> Collection<User> {
    dotenv().ok(); // Load environment variables from .env file

    let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    let client = Client::with_uri_str(&mongo_uri)
        .await
        .expect("Failed to initialize MongoDB client");
    let db = client.database("registration");
    db.collection::<User>("users")
}

// Email confirmation function
async fn send_confirmation_email(email: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load environment variables from .env file
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

// Create operation: Naya user add karne ka function
async fn create_user(user: web::Json<User>) -> impl Responder {
    let collection = get_db_collection().await;
    
    // Hash the password
    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(err) => {
            eprintln!("Password hashing error: {:?}", err);
            return HttpResponse::InternalServerError().body("Error hashing password");
        }
    };
    
    let mut new_user = user.into_inner();
    new_user.password = hashed_password; // Replace plain password with hashed password

    // Insert user into MongoDB
    match collection.insert_one(new_user, None).await {
        Ok(inserted) => {
            // Send confirmation email
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


// Login operation: User ko authenticate karne ka function
async fn login_user(user: web::Json<Userlogin>) -> impl Responder {
    let collection = get_db_collection().await;

    // Find user by username
    let filter = doc! { "username": &user.username };
    let result = collection.find_one(filter, None).await.unwrap();

    match result {
        Some(found_user) => {
            // Verify password
            if verify(&user.password, &found_user.password).unwrap() {
                // Generate JWT token
                let claims = found_user.clone();
                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret_key".as_ref())).unwrap(); // Use a strong secret key

                return HttpResponse::Ok().json(token);
            } else {
                return HttpResponse::Unauthorized().body("Invalid password");
            }
        }
        None => HttpResponse::NotFound().body("User not found"),
    }
}

// Password change struct for /forgot-password endpoint
#[derive(Debug, Deserialize,Clone)]
struct PasswordChangeRequest {
    username: String,
    old_password: String,
    new_password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Marks {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    student_id: String,
    subject1: i32,
    subject2: i32,
    teacher_name: String, // Teacher ka naam
    user_type: String, // User type to check if the user is a teacher
}

// Create operation: Marks add karne ka function
async fn add_marks(marks: web::Json<Marks>) -> impl Responder {
    // Check if the user is a teacher
    if marks.user_type != "teacher" {
        return HttpResponse::Unauthorized().body("Only teachers can add marks");
    }

    let collection = get_marks_collection().await;

    let new_marks = Marks {
        id: None,
        student_id: marks.student_id.clone(),
        subject1: marks.subject1,
        subject2: marks.subject2,
        teacher_name: marks.teacher_name.clone(),
        user_type: marks.user_type.clone(), // Store user type
    };

    // Marks ko collection mein insert karte hain
    match collection.insert_one(new_marks, None).await {
        Ok(inserted) => {
            HttpResponse::Created().json(inserted.inserted_id)
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to add marks")
        }
    }
}


// MongoDB se connection establish karne ke liye setup function
async fn get_marks_collection() -> Collection<Marks> {
    let client = Client::with_uri_str("mongodb+srv://chandanrust:chandanrust@rust-project.dhmtb.mongodb.net/").await.unwrap();
    let db = client.database("marks");
    db.collection::<Marks>("mark-list")
}

// Forgot password operation: Update user password
async fn forgot_password(request: web::Json<PasswordChangeRequest>) -> impl Responder {
    let collection = get_db_collection().await;

    // Find the user by username
    let filter = doc! { "username": &request.username };
    let result = collection.find_one(filter.clone(), None).await.unwrap();

    match result {
        Some(mut user) => {
            // Check if the old password matches
            if verify(&request.old_password, &user.password).unwrap() {
                // Check if the old password is the same as the new password
                if request.old_password == request.new_password {
                    return HttpResponse::BadRequest().body("New password cannot be the same as the old password.");
                }

                // Hash the new password
                let hashed_new_password = hash(&request.new_password, DEFAULT_COST).unwrap();
                user.password = hashed_new_password;

                // Update the password in MongoDB
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

async fn send_marks_added_email(student_email: &str, teacher_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load environment variables from .env file
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
        .to(student_email.parse()?)
        .subject("Marks Added Notification")
        .body(format!("Your teacher {} added your marks.", teacher_name))
        .unwrap();

    mailer.send(&email)?;

    Ok(())
}
// Actix Web ko setup karte hain aur endpoints define karte hain
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
   
    HttpServer::new(|| {
        App::new()
            .route("/register", web::post().to(create_user)) // Create user
            .route("/login", web::post().to(login_user))     // Login user
            .route("/forgot-password", web::post().to(forgot_password)) // Forgot password
            .route("/add-marks", web::post().to(add_marks)) // Add marks endpoint

    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
