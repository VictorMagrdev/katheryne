use serde::{Deserialize, Serialize};
use crate::domain::models::user::User;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Client {
    user: User
}

impl Client {
    pub fn new(
        first_name: String,
        last_name: String,
        username: String,
        password: String,
        phone: String,
        email: String,
    ) -> Self {
        let user:User = User::new(first_name, last_name, username, password, phone, email);
        Client {
            user
        }
    }
    pub fn accept_provider(&self, dni: String) -> bool {
        true
    }
}