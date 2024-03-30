use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::domain::models::client::Client;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub dni: Option<Thing>,
    first_name: String,
    last_name: String,
    username: String,
    password: String,
    phone: String,
    email: String,
}

impl User {
    pub fn new(
        first_name: String,
        last_name: String,
        username: String,
        password: String,
        phone: String,
        email: String,
    ) -> Self {
        User {
            dni: None,
            first_name,
            last_name,
            username,
            password,
            phone,
            email,
            }
        }
}


