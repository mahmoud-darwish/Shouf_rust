use axum::{
  Json, http::StatusCode, extract::{Path, State},
};

use crate::profile::models::{Profile, CreateProfile};
use chrono:: Utc;
use sqlx::PgPool;
use std::sync::Arc;

// POST /profiles - create new profile
pub async fn create_profile(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateProfile>,
) -> (StatusCode, Json<CreateProfile>) {
    let result = sqlx::query!(
        "INSERT INTO profile (user_id, name, avatar, bio, time_stamp) VALUES ($1, $2, $3, $4, $5)",
        payload.user_id,
        payload.name,
        payload.avatar,
        payload.bio,
        Utc::now()
    )
    .execute(&*pool)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, Json(payload)),
        Err(err) => {
            eprintln!("Error inserting profile: {:?}", err);  // Log the error for debugging
            (StatusCode::INTERNAL_SERVER_ERROR, Json(payload))
        }
    }
}
// GET /profiles - get all profiles
pub async fn get_profiles(State(pool): State<Arc<PgPool>>) -> Json<Vec<Profile>> {
    let profiles = sqlx::query_as::<_, Profile>("SELECT id, user_id, name, avatar, bio, time_stamp FROM profile")
        .fetch_all(&*pool)
        .await
        .expect("Failed to fetch profiles");

    Json(profiles)
}   

// GET /profiles/:id - get profile by id
pub async fn get_profile(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<i32>,
) -> Result<Json<Profile>, StatusCode> {
    let result = sqlx::query_as!(Profile, "SELECT id, user_id, name, avatar, bio, time_stamp FROM profile WHERE id = $1", id)
        .fetch_optional(&*pool)
        .await;

    match result {
        Ok(Some(profile)) => Ok(Json(profile)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// PUT /profiles/:id - update profile by id
pub async fn update_profile(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<i32>,
    Json(payload): Json<Profile>,
) -> Result<Json<Profile>, StatusCode> {
    let result = sqlx::query!(
        "UPDATE profile SET name = $1, avatar = $2, bio = $3, time_stamp = $4 WHERE id = $5",
        payload.name,
        payload.avatar,
        payload.bio,
        Utc::now(),
        id
    )
    .execute(&*pool)
    .await;

    match result {
        Ok(_) => {
            let updated_profile = Profile {
                id,
                user_id: payload.user_id,
                name: payload.name,
                avatar: payload.avatar,
                bio: payload.bio,
                time_stamp: Utc::now(),
            };
            Ok(Json(updated_profile))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// DELETE /profiles/:id - delete profile by id
pub async fn delete_profile(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<i32>,
) -> StatusCode {
    let result = sqlx::query!("DELETE FROM profile WHERE id = $1", id)
        .execute(&*pool)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
