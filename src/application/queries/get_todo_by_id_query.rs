use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use surrealdb::Error;
use crate::domain::models::todo::Todo;
use crate::infrastructure::data::repositories::todo_repository::TodoRepository;

pub async fn get_todo_by_id_query(Path(id): Path<String>)
                                  -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let repository:TodoRepository = TodoRepository::new();
    let id:String = id.to_string();

    let todo: Result<Todo, Error> = repository.get_by_id(id).await;

    return Ok((StatusCode::OK, Json(todo)));
}