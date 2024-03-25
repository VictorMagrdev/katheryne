use serde::{Deserialize, Serialize};
use crate::domain::models::user::User;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Provider {
    user: User,
    rating: f32,
}

impl Provider {
    pub fn calculate_rating(&self, ratings: Vec<i32>) -> f32 {
        0.0
    }

    pub fn accept_order(&self, id: String) -> bool {
        true
    }
}
