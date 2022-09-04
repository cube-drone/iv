use fred::pool::{RedisPool};

pub mod redis;
pub struct Services{
    pub local_redis_client: RedisPool,
    pub prime_redis_client: RedisPool,
}
