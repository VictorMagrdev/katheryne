use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use crate::domain::models::user::User;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Category {
    Remote,
    InPerson,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Status {
    Pending,
    Accepted,
    InProgress,
    Finished,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Order {
    id: Option<Thing>,
    title: String,
    status: Status,
    description: String,
    payment: f32,
    user: User,
    creation_date: Option<DateTime<Local>>,
    end_date: Option<DateTime<Local>>,
    availability_date: Option<DateTime<Local>>,
    category: Category
}
