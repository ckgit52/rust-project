// src/main.rs
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::sync::Arc;
use mongodb::{Client, Collection, options::ClientOptions};
use std::error::Error;
use futures::stream::TryStreamExt;
use actix_web::web::Data;

// Define the structure for our Item
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub price: f64,
}

// Define a struct to handle MongoDB operations
struct MongoRepo {
    collection: Collection<Item>,
}

impl MongoRepo {
    pub async fn init() -> Result<Self, Box<dyn Error>> {
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        let client = Client::with_options(client_options)?;
        let db = client.database("actix_db");
        let collection: Collection<Item> = db.collection("items");
        Ok(MongoRepo { collection })
    }

    pub async fn create_item(&self, item: Item) -> Result<(), Box<dyn Error>> {
        self.collection.insert_one(item, None).await?;
        Ok(())
    }

    pub async fn get_items(&self) -> Result<Vec<Item>, Box<dyn Error>> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        Ok(items)
    }

    pub async fn get_item(&self, id: &str) -> Result<Option<Item>, Box<dyn Error>> {
        let filter = mongodb::bson::doc! { "id": id };
        let item = self.collection.find_one(filter, None).await?;
        Ok(item)
    }

    pub async fn update_item(&self, id: &str, item: Item) -> Result<(), Box<dyn Error>> {
        let filter = mongodb::bson::doc! { "id": id };
        self.collection.replace_one(filter, item, None).await?;
        Ok(())
    }

    pub async fn delete_item(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let filter = mongodb::bson::doc! { "id": id };
        self.collection.delete_one(filter, None).await?;
        Ok(())
    }
}

// App state to share MongoRepo with handlers
#[derive(Clone)]
struct AppState {
    db: Arc<MongoRepo>,
}

// Define the structure for item requests
#[derive(Serialize, Deserialize)]
struct ItemRequest {
    name: String,
    description: String,
    price: f64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the MongoDB repository
    let db = MongoRepo::init().await.expect("Failed to connect to MongoDB");
    let data = Data::new(AppState {
        db: Arc::new(db),
    });

    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(web::resource("/items")
                .route(web::post().to(create_item))
                .route(web::get().to(get_items))
            )
            .service(web::resource("/items/{id}")
                .route(web::get().to(get_item))
                .route(web::put().to(update_item))
                .route(web::delete().to(delete_item))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Handler for creating an item
async fn create_item(data: Data<AppState>, item_req: web::Json<ItemRequest>) -> impl Responder {
    let item = Item {
        id: Some(Uuid::new_v4().to_string()),
        name: item_req.name.clone(),
        description: item_req.description.clone(),
        price: item_req.price,
    };

    match data.db.create_item(item).await {
        Ok(_) => HttpResponse::Created().json("Item created"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Handler for retrieving all items
async fn get_items(data: Data<AppState>) -> impl Responder {
    match data.db.get_items().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Handler for retrieving a single item by ID
async fn get_item(data: Data<AppState>, id: web::Path<String>) -> impl Responder {
    match data.db.get_item(&id).await {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().json("Item not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Handler for updating an item
async fn update_item(data: Data<AppState>, id: web::Path<String>, item_req: web::Json<ItemRequest>) -> impl Responder {
    let updated_item = Item {
        id: Some(id.to_string()),
        name: item_req.name.clone(),
        description: item_req.description.clone(),
        price: item_req.price,
    };

    match data.db.update_item(&id, updated_item).await {
        Ok(_) => HttpResponse::Ok().json("Item updated"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Handler for deleting an item
async fn delete_item(data: Data<AppState>, id: web::Path<String>) -> impl Responder {
    match data.db.delete_item(&id).await {
        Ok(_) => HttpResponse::Ok().json("Item deleted"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
