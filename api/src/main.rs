use std::sync::Arc;
use std::env;
use log::{debug, error, log_enabled, info, Level};

use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use axum::http::StatusCode;

use fred::prelude::{RedisError, KeysInterface};
use fred::pool::{RedisPool};

pub mod services;
use services::redis::connect_redis;
use services::Services;

#[tokio::main]
async fn main() {

    // boot up yon logger
    env_logger::init();

    info!("Booting up the application!");

    let redis_local_url = match env::var("IV_REDIS_LOCAL_URL") {
        Ok(val) => val,
        Err(_e) => String::from("redis://127.0.0.1:6379"),
    };
    let redis_prime_url = match env::var("IV_REDIS_PRIME_URL") {
        Ok(val) => val,
        Err(_e) => String::from("redis://127.0.0.1:6379"),
    };
    let bind = match env::var("IV_BIND") {
        Ok(val) => val,
        Err(_e) => String::from("0.0.0.0:4000"),
    };
    
    info!("Connecting to redis...");
    
    let connect_result: Result<RedisPool, RedisError> = connect_redis(&redis_local_url).await;
    let client = connect_result.unwrap();
    
    let connect_result: Result<RedisPool, RedisError> = connect_redis(&redis_prime_url).await;
    let client_prime = connect_result.unwrap();

    let shared_state: Arc<Services> = Arc::new(Services {
        local_redis_client: client,
        prime_redis_client: client_prime,
    });

    info!("Connected!");
    
    // build our application with a single route
    let app = Router::new()
        .route("/", get(handler))
        .route("/identify", get(identify))
        .layer(Extension(shared_state));

    // run it with hyper on localhost:3000
    axum::Server::bind(&bind.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(
    Extension(state): Extension<Arc<Services>>,
) -> Result<String, (StatusCode, String)> {
    //let counter:i32 = state.local_redis_client.incr("counter").await.unwrap();
    info!("hitting the 404 endpoint!");

    //format!("Hit Counter: {}", counter)
    Err((StatusCode::NOT_FOUND, String::from("dave's not here man")))
}

async fn identify(
    Extension(state): Extension<Arc<Services>>,
) -> Result<String, (StatusCode, String)> {
    // hitting "identify" with no args creates a blank identity
    
    info!("hitting the identify endpoint!");

    let counter:i32 = match state.local_redis_client.incr("counter").await{
        Ok(counter) => counter,
        Err(_e) => {
            // log here
            return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Critical redis error!")))
        },
    };

    Ok(format!("Hit Counter: {}", counter))
}