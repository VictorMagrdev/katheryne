use axum::{http::StatusCode, Json, response::IntoResponse};
use chrono::{DateTime, Local};
use serde_json::Value;
use crate::domain::models::client::Client;
use crate::domain::models::provider::Provider;

use crate::domain::models::todo::Todo;
use crate::infrastructure::data::repositories::repository::Repository;
use crate::infrastructure::data::repositories::todo_repository::TodoRepository;

pub async fn create_provider_command(Json(mut body): Json<Provider>)
                                   -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let repository: Repository = Repository::new("provider");

    let provider:Provider = body.to_owned();

    let provider:Provider = repository.create_repository(provider.clone()).await.unwrap()[0].to_owned();

    let json_response: Value = serde_json::json!({
        "status": "success".to_string(),
        "data": todo,
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}