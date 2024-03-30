use axum::Json;
use axum::response::IntoResponse;
use serde_json::Value;
use crate::infrastructure::data::repositories::repository::Repository;

pub async fn get_all_query<T>(table: &str) -> impl IntoResponse {
    let repository:Repository = Repository::new(table);

    let mut query: Vec<T> = Vec::new();
    if let Ok(result) = repository.get_all().await {
        query = result;
    }

    let json_response:Value = serde_json::json!({
        "status": "success",
        "results": todos.len(),
        "todos": todos,
    });

    Json(json_response)
}