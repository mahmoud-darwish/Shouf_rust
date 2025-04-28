use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;


#[derive(Serialize, Deserialize, FromRow)]
pub struct Channel{
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub logo: String,
    pub bio: String,
    pub time_stamp: DateTime<Utc>
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateChannel{
    pub user_id: i32,
    pub name: String,
    pub logo: String,
    pub bio: String
}
