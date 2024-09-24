use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod handlers;
mod db;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables

    std::env::set_var("RUST_LOG", "actix_web=debug");

    HttpServer::new(|| {
        App::new()
            .route("/register", web::post().to(handlers::create_user)) // Create user
            .route("/login", web::post().to(handlers::login_user))     // Login user
            .route("/forgot-password", web::post().to(handlers::forgot_password)) // Forgot password
            .route("/add-marks", web::post().to(handlers::add_marks)) // Add marks endpoint
            .route("/edit-mark", web::post().to(handlers::edit_marks::edit_mark)) // Edit marks route
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
