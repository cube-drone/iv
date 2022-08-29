use std::sync::Arc;

use axum::{
    extract::Extension,
    routing::get,
    Router,
};

use fred::prelude::{RedisError, RedisConfig, KeysInterface};
use fred::pool::{RedisPool};

const CACHE_POOL_SIZE: usize = 2;

pub async fn connect() -> Result<RedisPool, RedisError> {
    let config = RedisConfig::from_url("redis://four@127.0.0.1:41001")?;
    //redis|rediss[-cluster] :// [[username:]password@] host [:port][?[node=host1:port1][&node=host2:port2][&node=hostN:portN]]
    let pool = RedisPool::new(config, CACHE_POOL_SIZE)?;
    let _ = pool.connect(None);
    let _ = pool.wait_for_connect().await?;
  
    Ok(pool)
}

#[tokio::main]
async fn main() {
    let connect_result: Result<RedisPool, RedisError> = connect().await;
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