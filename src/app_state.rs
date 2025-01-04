use crate::commons::{
    authentication::auth_keys_service::{AuthKeys, Authenticator},
    repositories::base::Repository
};

pub struct AppState<T: Repository, U: Authenticator> {
    pub repo: T,
    pub auth_service: U,
    pub auth_keys: AuthKeys
}
