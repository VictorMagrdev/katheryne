use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::Value;
use crate::infrastructure::data::repositories::todo_repository::TodoRepository;

pub async fn delete_todo_command( Path(id): Path<String>)
    -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {

    let repository:TodoRepository = TodoRepository::new();
    let id:String = id.to_string();

    if let Ok(_) = repository.get_by_id(id.clone()).await {
        let _ = repository.delete_todo(id.clone()).await.unwrap();

        return Ok(StatusCode::NO_CONTENT);
    }

    let error_response:Value = serde_json::json!({
        "status": "error",
        "message": format!("Todo with ID: {} not found", id)
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}