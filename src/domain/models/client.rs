use serde::{Deserialize, Serialize};
use crate::domain::models::user::User;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Client {
    user: User,
}

impl Client {
    pub fn accept_provider(&self, dni: String) -> bool {
        true
    }
}