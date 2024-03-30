use serde::{Deserialize, Serialize};
use crate::domain::models::client::Client;
use crate::domain::models::user::User;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Provider {
    user: User,
    rating: f32,
}

impl Provider {
    pub fn new(
        first_name: String,
        last_name: String,
        username: String,
        password: String,
        phone: String,
        email: String,
        rating: f32
    ) -> Self {
        let user:User = User::new(first_name, last_name, username, password, phone, email);
        Provider {
            user,
            rating
        }
    }
    pub fn calculate_rating(&self, ratings: Vec<i32>) -> f32 {
        0.0
    }

    pub fn accept_order(&self, id: String) -> bool {
        true
    }
}
