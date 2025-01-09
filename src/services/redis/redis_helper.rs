use async_trait::async_trait;
use crate::services::redis::redis_service::{RedisConnGetter, RedisSvc};

#[async_trait]
pub trait RedisHelper {
    async fn get(&self, key: String) -> Result<String, ()>;
    async fn set(&self, key: String, val: String) -> Result<bool, ()>;
}

#[async_trait]
impl RedisHelper for RedisSvc {
    async fn get(&self, key: String) -> Result<String, ()> {
        let mut conn = self.get_conn().clone();
        let tes = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await;
        Ok(tes.unwrap())
    }

    async fn set(&self, key: String, val: String) -> Result<bool, ()> {
        let mut conn = self.get_conn().clone();
        let result = redis::cmd("SET")
            .arg(&key)
            .arg(&val)
            .exec_async(&mut conn)
            .await;

        match result {
            Ok(()) => {
                Ok(true)
            }
            Err(_) => {
                Ok(false)
            }
        }
    }

}
