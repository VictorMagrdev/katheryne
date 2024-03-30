use axum::{http::StatusCode, Json, response::IntoResponse};
use chrono::{DateTime, Local};
use serde_json::Value;
use crate::domain::models::client::Client;

use crate::domain::models::todo::Todo;
use crate::infrastructure::data::repositories::repository::Repository;
use crate::infrastructure::data::repositories::todo_repository::TodoRepository;

pub async fn create_client_command(Json(mut body): Json<Client>)
                                 -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let repository: Repository = Repository::new("client");

    let client:Client = body.to_owned();

    let client: Client = repository.create_repository(client.clone()).await.unwrap()[0].to_owned();

    let json_response: Value = serde_json::json!({
        "status": "success".to_string(),
        "data": todo,
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}