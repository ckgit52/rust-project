use mongodb::{Client, Collection};
use crate::models::{User, Marks};
use std::env;

pub async fn get_db_collection() -> Collection<User> {
    let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&mongo_uri)
        .await
        .expect("Failed to initialize MongoDB client");
    
    let db = client.database("registration");
    db.collection::<User>("users")
}

pub async fn get_marks_collection() -> Collection<Marks> {
    let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&mongo_uri)
        .await
        .expect("Failed to initialize MongoDB client");

    let db = client.database("marks");
    db.collection::<Marks>("mark-list")
}
