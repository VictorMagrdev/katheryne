use axum::Json;
use axum::response::IntoResponse;
use serde_json::Value;
use crate::domain::models::todo::Todo;
use crate::infrastructure::data::repositories::todo_repository::TodoRepository;

pub async fn get_all_todos_query() -> impl IntoResponse {
    let repository:TodoRepository = TodoRepository::new();

    let mut todos: Vec<Todo> = Vec::new();
    if let Ok(result) = repository.get_all().await {
        todos = result;
    }

    let json_response:Value = serde_json::json!({
        "status": "success",
        "results": todos.len(),
        "todos": todos,
    });

    Json(json_response)
}