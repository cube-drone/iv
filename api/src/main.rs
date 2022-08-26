use std::time::Duration;
use std::sync::Arc;

use axum::{
    extract::Extension,
    routing::get,
    Router,
};

use mobc::async_trait;
use mobc::{Pool, Connection, Manager};
use redis::{ErrorKind, RedisError, AsyncCommands, RedisResult};
use redis::cluster::{ClusterClient, ClusterConnection};

pub struct RedisConnectionManager {
    client: ClusterClient,
}

impl RedisConnectionManager {
    pub fn new(c: ClusterClient) -> Self {
        Self { client: c }
    }
}

pub type MobcPool = Pool<RedisConnectionManager>;
pub type MobcCon = Connection<RedisConnectionManager>;

#[async_trait]
impl Manager for RedisConnectionManager {
    type Connection = ClusterConnection;
    type Error = RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let c = self.client.get_connection()?;
        Ok(c)
    }

    async fn check(&self, mut conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        let pong: String = redis::cmd("PING").query_async(&mut conn).await?;
        if pong.as_str() != "PONG" {
            return Err((ErrorKind::ResponseError, "pong response error").into());
        }
        Ok(conn)
    }
}

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

pub async fn connect() -> Result<MobcPool, RedisError> {
    let nodes = vec![
        "redis://:bitnami@127.0.0.1:41000/", 
        "redis://:bitnami@127.0.0.1:41001/", 
        "redis://:bitnami@127.0.0.1:41002/", 
        "redis://:bitnami@127.0.0.1:41003/", 
        "redis://:bitnami@127.0.0.1:41004/", 
        "redis://:bitnami@127.0.0.1:41005/"];
    let client = ClusterClient::open(nodes).unwrap();
    let manager = RedisConnectionManager::new(client);
    Ok(Pool::builder()
        .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
        .max_open(CACHE_POOL_MAX_OPEN)
        .max_idle(CACHE_POOL_MAX_IDLE)
        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
        .build(manager))
}

#[tokio::main]
async fn main() {
    let connect_result: Result<MobcPool, RedisError> = connect().await;
    let client = connect_result.unwrap();
    let shared_pool = Arc::new(client);

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
    Extension(redis_pool): Extension<Arc<MobcPool>>,
) -> String {
    let mut con:MobcCon = redis_pool.get().await.unwrap();

    let counter_res:RedisResult<i32> = con.incr("counter", 1i32).await;
    let counter = counter_res.unwrap();

    format!("Hit Counter: {}", counter)
}