mod db;
mod todo;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get, post, put},
    Json, Router,
};
use db::Database;
use serde::Deserialize;
use std::sync::Mutex;
use std::{fs, net::SocketAddr, sync::Arc};
use todo::Todo;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Database>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        db: Arc::new(Mutex::new(Database::load())),
    };

    let app = Router::new()
        .route("/", get(root))
        .nest(
            "/api",
            Router::new()
                // routing for add_todo, list_todos and delete_todo
                .route("/todos", post(add_todo).get(list_todos))
                .route("/todos/:id", delete(delete_todo))
                .route("/todos/:id/toggle", put(toggle_todo))
                .route("/todos/clear-done", delete(clear_done)),
        )
        .layer(Extension(state))
        .nest_service("/static", ServeDir::new("static"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn root() -> Html<String> {
    Html(fs::read_to_string("views/index.html").unwrap())
}

#[derive(Deserialize)]
struct FormTodo {
    content: String,
}

async fn add_todo(
    Extension(state): Extension<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let todo: FormTodo = serde_json::from_value(payload).unwrap();
    let todo = Todo::new(&todo.content);

    let mut db = state.db.lock().unwrap();
    db.add(todo.clone());
    db.save();

    (StatusCode::CREATED, Json(todo))
}

async fn list_todos(Extension(state): Extension<AppState>) -> impl IntoResponse {
    let db = state.db.lock().unwrap();
    let todos = db.todos().clone();

    Json(todos)
}

async fn delete_todo(
    Extension(state): Extension<AppState>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let mut db = state.db.lock().unwrap();

    match db.delete(id) {
        Some(_) => (),
        None => return StatusCode::NOT_FOUND,
    };

    db.save();

    StatusCode::OK
}

async fn toggle_todo(
    Extension(state): Extension<AppState>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let mut db = state.db.lock().unwrap();
    match db.toggle(id) {
        Some(_) => (),
        None => return StatusCode::NOT_FOUND,
    };

    db.save();

    StatusCode::OK
}

async fn clear_done(Extension(state): Extension<AppState>) -> impl IntoResponse {
    let mut db = state.db.lock().unwrap();
    db.clear_done();
    db.save();

    StatusCode::OK
}
