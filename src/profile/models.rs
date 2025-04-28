use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Profile {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub avatar: String,
    pub bio: String,
    pub time_stamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateProfile {
    pub user_id: i32,
    pub name: String,
    pub avatar: String,
    pub bio: String,
}