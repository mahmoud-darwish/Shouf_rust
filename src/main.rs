mod channel;
mod profile;

use axum::{Router};
use sqlx::PgPool;
use std::net::SocketAddr;
use channel::handlers as channel_handlers;
use profile::handlers as profile_handlers;
use dotenv::dotenv;
use std::sync::Arc;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = "postgresql://mahmoudibrahim:kokowawa@localhost:5432/shouf";
    let pool = PgPool::connect(database_url)
        .await
        .expect("Failed to connect to the database");

    // Wrap the pool in an Arc for shared state
    let shared_pool = Arc::new(pool);
    println!("Shared pool type: {:?}", std::any::type_name::<Arc<PgPool>>());
    let app = Router::new()
        .route("/channels", axum::routing::post(channel_handlers::create_channel).get(channel_handlers::get_channels))
        .route("/channels/:id", axum::routing::get(channel_handlers::get_channels).delete(channel_handlers::delete_channel).put(channel_handlers::update_channel))
        .route("/profiles", axum::routing::post(profile_handlers::create_profile).get(profile_handlers::get_profiles))
        .route("/profiles/:id", axum::routing::get(profile_handlers::get_profile).delete(profile_handlers::delete_profile).put(profile_handlers::update_profile))
        .with_state(shared_pool);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}