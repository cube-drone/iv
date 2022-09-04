
use fred::prelude::{RedisError, RedisConfig};
use fred::pool::{RedisPool};
use fred::types::ReconnectPolicy;


const CACHE_POOL_SIZE: usize = 2;

pub async fn connect_redis(url: &String) -> Result<RedisPool, RedisError> {
    let config = RedisConfig::from_url(url)?;
    //redis|rediss[-cluster] :// [[username:]password@] host [:port][?[node=host1:port1][&node=host2:port2][&node=hostN:portN]]
    let pool = RedisPool::new(config, CACHE_POOL_SIZE)?;
    let policy = ReconnectPolicy::Constant { 
        attempts: 0, 
        max_attempts: 0, 
        delay: 1000, 
        jitter: 50 
    };
    let _ = pool.connect(Some(policy));
    let _ = pool.wait_for_connect().await?;

    Ok(pool)
}