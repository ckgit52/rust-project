use mongodb::{Client, Collection};
use std::env;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use crate::models::marks::Marks;

pub static MONGO_CLIENT: Lazy<Mutex<Option<Client>>> = Lazy::new(|| Mutex::new(None));

pub async fn get_marks_collection() -> Collection<Marks> {
    let mut client_lock = MONGO_CLIENT.lock().await;

    if client_lock.is_none() {
        // MongoDB URI from the .env file
        let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

        // Initialize MongoDB client
        let client = Client::with_uri_str(&mongo_uri)
            .await
            .expect("Failed to initialize MongoDB client");
        
        *client_lock = Some(client);
    }

    // Return the collection
    let client = client_lock.as_ref().unwrap();
    let db = client.database("marks");
    db.collection::<Marks>("mark-list")
}
