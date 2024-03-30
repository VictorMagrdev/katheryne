use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::Value;
use crate::infrastructure::data::repositories::repository::Repository;

pub async fn delete_provider_command( Path(id): Path<String>)
                                   -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {

    let repository:Repository = Repository::new("provider");
    let id:String = id.to_string();

    if let Ok(_) = repository.get_by_id(id.clone()).await {
        let _ = repository.delete(id.clone()).await.unwrap();

        return Ok(StatusCode::NO_CONTENT);
    }

    let error_response:Value = serde_json::json!({
        "status": "error",
        "message": format!("Provider with ID: {} not found", id)
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}