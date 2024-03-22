use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    routing::{delete, get},
    Form, Router,
};
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    todos: Arc<Mutex<Vec<String>>>,
}
pub fn create_router() -> Router {
    let state = AppState {
        todos: Arc::new(Mutex::new(vec![
            String::from("Buy milk"),
            String::from("Walk the dog"),
        ])),
    };
    let router = Router::new()
        .route("/", get(todo).post(add_todo))
        .route("/:todo", delete(delete_todo))
        .route("/clear", get(|| async { TodoField }));

    router.with_state(state)
}

#[derive(Template)]
#[template(path = "pages/todos.html")]
struct HelloTemplate {
    todos: Vec<String>,
}

async fn todo(State(state): State<AppState>) -> HelloTemplate {
    let todos = state.todos.lock().await.clone();
    HelloTemplate { todos }
}

#[derive(serde::Deserialize)]
struct AddTodo {
    todo: String,
}

#[derive(Template)]
#[template(path = "components/todo_list.html")]
struct TodoListTemplate {
    todos: Vec<String>,
}

#[derive(Template)]
#[template(path = "components/todo_field.html")]
struct TodoField;

async fn add_todo(State(state): State<AppState>, Form(form): Form<AddTodo>) -> TodoListTemplate {
    let todo = form.todo;
    let mut todos = state.todos.lock().await;
    todos.push(todo);

    TodoListTemplate {
        todos: todos.clone(),
    }
}

async fn delete_todo(State(state): State<AppState>, Path(todo): Path<String>) {
    let mut todos = state.todos.lock().await;
    if let Some(index) = todos.iter().position(|t| *t == todo) {
        todos.remove(index);
    }
}
