use crate::commons::{
    authentication::auth_keys_service::{AuthKeys, Authenticator},
    repositories::base::Repository
};
use crate::commons::instrumentation::statsd_config::StatsdService;
use crate::services::redis::redis_service::RedisService;

pub struct AppState<T: Repository, U: Authenticator, V: RedisService, W: StatsdService> {
    pub repo: T,
    pub auth_service: U,
    pub auth_keys: AuthKeys,
    pub redis_service: V,
    pub statsd_service: W,
}
