use mongodb::{Client, Collection};
use crate::models::Marks;
use std::env;

// This function establishes a connection to the MongoDB collection for Marks
pub async fn get_marks_collection() -> Collection<Marks> {
    // MongoDB URI from the .env file
    let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    // Initialize MongoDB client
    let client = Client::with_uri_str(&mongo_uri)
        .await
        .expect("Failed to initialize MongoDB client");

    // Access the database and the collection
    let db = client.database("marks");
    db.collection::<Marks>("mark-list")
}
