use axum::{
     Json, http::StatusCode, extract::{Path, State},
};

use crate::channel::models::{Channel, CreateChannel};
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;

pub async fn create_channel(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateChannel>,
) -> (StatusCode, Json<CreateChannel>) {
    let result = sqlx::query!(
        "INSERT INTO channel (user_id, name, logo, bio, time_stamp) VALUES ($1, $2, $3, $4, $5)",
        payload.user_id,
        payload.name,
        payload.logo,
        payload.bio,
        Utc::now()
    )
    .execute(&*pool)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, Json(payload)),
        Err(err) => {
            eprintln!("Error inserting Channel: {:?}", err); 
            (StatusCode::INTERNAL_SERVER_ERROR, Json(payload))
        }
    }
}

pub  async fn get_channels(State(pool): State<Arc<PgPool>>) -> Json<Vec<Channel>> {
    let channels = sqlx::query_as!(Channel, "SELECT * FROM channel")
        .fetch_all(&*pool)
        .await
        .expect("Failed to fetch channels");

    Json(channels)
}  

pub async fn update_channel(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<i32>,
    Json(payload): Json<Channel>,
) -> Result<Json<Channel>, StatusCode> {
    let result = sqlx::query!(
        "UPDATE channel SET name = $1, logo = $2, bio = $3, time_stamp = $4 WHERE id = $5",
        payload.name,
        payload.logo,
        payload.bio,
        Utc::now(),
        id
    )
    .execute(&*pool)
    .await;

    match result {
        Ok(_) => {
            let updated_channel = Channel {
                id,
                user_id: payload.user_id,
                name: payload.name,
                logo: payload.logo,
                bio: payload.bio,
                time_stamp: Utc::now(),
            };
            Ok(Json(updated_channel))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub  async fn delete_channel(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<i32>,
) -> StatusCode {
    let result = sqlx::query!("DELETE FROM channel WHERE id = $1", id)
        .execute(&*pool)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}