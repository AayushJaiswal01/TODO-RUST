mod model;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use model::{CreateTodo, Todo, UpdateTodo};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

type AppState = Arc<Mutex<Vec<Todo>>>;

#[tokio::main]
async fn main() {
    let shared_state = AppState::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/todos", post(create_todo_handler))
        .route("/todos", get(list_todos_handler))
        .route("/todos/:id", get(get_todo_handler))
        .route("/todos/:id", put(update_todo_handler))
        .route("/todos/:id", delete(delete_todo_handler))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Hello, Axum World! This is the To-Do API root."
}

async fn create_todo_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    println!("POST /todos - Received new todo request: {:?}", payload);

    let new_todo = Todo {
        id: Uuid::new_v4(),
        text: payload.text,
        completed: payload.completed,
    };

    let mut todos_list = state.lock().await;
    todos_list.push(new_todo.clone());

    println!("POST /todos - Added new todo: {:?}", new_todo);
    println!("POST /todos - Current todos count: {}", todos_list.len());

    (StatusCode::CREATED, Json(new_todo))
}

async fn list_todos_handler(
    State(state): State<AppState>,
) -> (StatusCode, Json<Vec<Todo>>) {
    println!("GET /todos - Listing all todos");

    let todos_list = state.lock().await;

    println!("GET /todos - Current todos count: {}", todos_list.len());

    (StatusCode::OK, Json(todos_list.clone()))
}

async fn get_todo_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    println!("GET /todos/:id - Received request for id: {}", id);

    let todos_list = state.lock().await;

    if let Some(todo) = todos_list.iter().find(|t| t.id == id) {
        println!("GET /todos/:id - Found todo: {:?}", todo);
        Ok((StatusCode::OK, Json(todo.clone())))
    } else {
        println!("GET /todos/:id - Todo with id {} not found", id);
        Err(StatusCode::NOT_FOUND)
    }
}

async fn update_todo_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    println!("PUT /todos/:id - Received update request for id: {} with payload: {:?}", id, payload);

    let mut todos_list = state.lock().await;

    if let Some(index) = todos_list.iter().position(|t| t.id == id) {
        let todo_to_update = &mut todos_list[index];

        if let Some(new_text) = payload.text {
            todo_to_update.text = new_text;
        }
        if let Some(new_completed_status) = payload.completed {
            todo_to_update.completed = new_completed_status;
        }

        println!("PUT /todos/:id - Updated todo: {:?}", todo_to_update);
        Ok((StatusCode::OK, Json(todo_to_update.clone())))
    } else {
        println!("PUT /todos/:id - Todo with id {} not found for update", id);
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_todo_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    println!("DELETE /todos/:id - Received delete request for id: {}", id);

    let mut todos_list = state.lock().await;

    let initial_len = todos_list.len();
    todos_list.retain(|t| t.id != id);

    if todos_list.len() < initial_len {
        println!("DELETE /todos/:id - Deleted todo with id: {}", id);
        println!("DELETE /todos/:id - Current todos count: {}", todos_list.len());
        StatusCode::NO_CONTENT
    } else {
        println!("DELETE /todos/:id - Todo with id {} not found for deletion", id);
        StatusCode::NOT_FOUND
    }
}