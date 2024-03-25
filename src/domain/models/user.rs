use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    dni: Option<Thing>,
    first_name: String,
    last_name: String,
    username: String,
    password: String,
    phone: String,
    email: Vec<String>,
}