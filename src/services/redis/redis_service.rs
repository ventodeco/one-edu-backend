use std::env;
use async_trait::async_trait;
use dotenv::dotenv;
use redis::aio::MultiplexedConnection;

#[async_trait]
pub trait RedisService {
    async fn init() -> Self;
}

#[derive(Debug, Clone)]
pub struct RedisSvc {
    redis_conn: MultiplexedConnection
}

#[async_trait]
impl RedisService for RedisSvc {
    async fn init() -> Self {
        RedisSvc {
            redis_conn: get_redis_conn().await
        }
    }
}

pub trait RedisConnGetter: RedisService {
    type Output;

    fn get_conn(&self) -> &Self::Output;
}

impl RedisConnGetter for RedisSvc {
    type Output = MultiplexedConnection;

    fn get_conn(&self) -> &Self::Output {
        &self.redis_conn
    }
}

async fn get_redis_conn() -> MultiplexedConnection {
    dotenv().ok();

    let host = env::var("REDIS_HOST").unwrap();
    let port = env::var("REDIS_PORT").unwrap();
    let client = redis::Client::open(format!("redis://{}:{}", host, port)).unwrap();
    client.get_multiplexed_async_connection().await.unwrap()
}
