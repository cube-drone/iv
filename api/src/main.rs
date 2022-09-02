use std::sync::Arc;
use std::env;

use axum::{
    extract::Extension,
    routing::get,
    Router,
};

use fred::prelude::{RedisError, KeysInterface};
use fred::pool::{RedisPool};

pub mod services;
use services::redis::connect_redis;

struct State{
    redis_client: RedisPool,
}

#[tokio::main]
async fn main() {
    let redis_url = match env::var("IV_REDIS_URL") {
        Ok(val) => val,
        Err(_e) => String::from("redis://127.0.0.1:6379"),
    };
    let bind = match env::var("IV_BIND") {
        Ok(val) => val,
        Err(_e) => String::from("0.0.0.0:4000"),
    };
    let connect_result: Result<RedisPool, RedisError> = connect_redis(&redis_url).await;
    let client = connect_result.unwrap();
    let shared_state: Arc<State> = Arc::new(State {
        redis_client: client,
    });

    // build our application with a single route
    let app = Router::new()
        .route("/", get(handler))
        .layer(Extension(shared_state));

    // run it with hyper on localhost:3000
    axum::Server::bind(&bind.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(
    Extension(state): Extension<Arc<State>>,
) -> String {
    let counter:i32 = state.redis_client.incr("counter").await.unwrap();

    format!("Hit Counter: {}", counter)
}