use axum::{http::StatusCode, Json, response::IntoResponse};
use chrono::{DateTime, Local};
use serde_json::Value;
use crate::domain::models::order::Order;
use crate::infrastructure::data::repositories::repository::Repository;
use crate::infrastructure::data::repositories::todo_repository::TodoRepository;

pub async fn create_order_command(Json(mut body): Json<Order>)
                                     -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let repository: Repository = Repository::new("order");

    let order:Order = body.to_owned();

    let order:Order = repository.create_repository(order.clone()).await.unwrap()[0].to_owned();

    let json_response: Value = serde_json::json!({
        "status": "success".to_string(),
        "data": todo,
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}