use std::sync::Arc;

use axum::{
    extract::Extension,
    routing::get,
    Router,
};

use fred::prelude::{RedisError, KeysInterface};
use fred::pool::{RedisPool};

pub mod services;
use services::redis::connect_redis;

#[tokio::main]
async fn main() {
    let connect_result: Result<RedisPool, RedisError> = connect_redis().await;
    let client = connect_result.unwrap();
    let shared_pool: Arc<RedisPool> = Arc::new(client);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(handler))
        .layer(Extension(shared_pool));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(
    Extension(redis_pool): Extension<Arc<RedisPool>>,
) -> String {
    let counter:i32 = redis_pool.incr("counter").await.unwrap();

    format!("Hit Counter: {}", counter)
}