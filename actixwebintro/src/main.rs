// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use serde::{Deserialize, Serialize};
// use std::sync::Mutex;

// // Struct for the ToDo item
// #[derive(Serialize, Deserialize, Clone)]
// struct ToDoItem {
//     id: usize,
//     title: String,
//     completed: bool,
// }

// // Application state to store the list of ToDo items
// struct AppState {
//     todo_list: Mutex<Vec<ToDoItem>>,
// }

// // Handler to get all ToDo items
// async fn get_todos(data: web::Data<AppState>) -> impl Responder {
//     let todos = data.todo_list.lock().unwrap();
//     HttpResponse::Ok().json(&*todos)
// }

// // Handler to add a new ToDo item
// async fn add_todo(
//     data: web::Data<AppState>,
//     new_todo: web::Json<ToDoItem>,
// ) -> impl Responder {
//     let mut todos = data.todo_list.lock().unwrap();
//     todos.push(new_todo.into_inner());
//     HttpResponse::Ok().json("Task added")
// }

// // Handler to mark a ToDo item as completed
// async fn mark_completed(
//     data: web::Data<AppState>,
//     path: web::Path<usize>,
// ) -> impl Responder {
//     let id = path.into_inner();
//     let mut todos = data.todo_list.lock().unwrap();
    
//     if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
//         todo.completed = true;
//         HttpResponse::Ok().json(format!("Task {} marked as completed", id))
//     } else {
//         HttpResponse::NotFound().json(format!("Task {} not found", id))
//     }
// }

// // Handler to delete a ToDo item
// async fn delete_todo(
//     data: web::Data<AppState>,
//     path: web::Path<usize>,
// ) -> impl Responder {
//     let id = path.into_inner();
//     let mut todos = data.todo_list.lock().unwrap();
    
//     if todos.iter().any(|todo| todo.id == id) {
//         todos.retain(|todo| todo.id != id);
//         HttpResponse::Ok().json(format!("Task {} deleted", id))
//     } else {
//         HttpResponse::NotFound().json(format!("Task {} not found", id))
//     }
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let app_state = web::Data::new(AppState {
//         todo_list: Mutex::new(Vec::new()),
//     });

//     HttpServer::new(move || {
//         App::new()
//             .app_data(app_state.clone()) // Share state across handlers
//             .route("/todos", web::get().to(get_todos)) // Get all tasks
//             .route("/todos", web::post().to(add_todo)) // Add a new task
//             .route("/todos/{id}/complete", web::put().to(mark_completed)) // Mark task as complete
//             .route("/todos/{id}", web::delete().to(delete_todo)) // Delete a task
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

use actix_web::{web::{self, route}, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello, Worlddddddddd! from chandan "
}
async fn printname() -> impl Responder{
    return "here name will b printed"
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index)) // Route for the index handler
            
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}



