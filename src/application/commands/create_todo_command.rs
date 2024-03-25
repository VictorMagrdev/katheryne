use axum::{http::StatusCode, Json, response::IntoResponse};
use chrono::{DateTime, Local};
use serde_json::Value;

use crate::domain::models::todo::Todo;
use crate::infrastructure::data::repositories::todo_repository::TodoRepository;

pub async fn create_todo_command(Json(mut body): Json<Todo>)
                                 -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let repository: TodoRepository = TodoRepository::new();

    if let Ok(todo) = repository.get_by_title(body.title.clone()).await {
        let json_response: Value = serde_json::json!({
            "status": "error".to_string(),
            "message": "Todo already exists".to_string(),
            "data": todo,
        });

        return Err((StatusCode::BAD_REQUEST, Json(json_response)));
    }

    let datetime: DateTime<Local> = Local::now();
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);
    let todo: Todo = body.to_owned();

    let todo: Todo = repository.create_todo(todo.clone()).await.unwrap()[0].to_owned();

    let json_response: Value = serde_json::json!({
        "status": "success".to_string(),
        "data": todo,
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}