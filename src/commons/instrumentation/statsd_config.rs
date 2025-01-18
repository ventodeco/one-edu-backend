use std::env;
use async_trait::async_trait;
use statsd::Client;

#[async_trait]
pub trait StatsdService {
    async fn init() -> Self;
}

pub struct StatsdSvc {
    client: Client,
}

#[async_trait]
impl StatsdService for StatsdSvc {
    async fn init() -> Self {
        let client = Client::new(&format!("{}:{}", env::var("STATSD_HOST").unwrap(), env::var("STATSD_PORT").unwrap()), &*env::var("STATSD_PREFIX").unwrap()).unwrap();
        StatsdSvc { client }
    }
}

pub trait StatsdClientGetter: StatsdService {
    type Output;

    fn get_client(&self) -> &Self::Output;
}

impl StatsdClientGetter for StatsdSvc {
    type Output = Client;

    fn get_client(&self) -> &Self::Output {
        &self.client
    }
}