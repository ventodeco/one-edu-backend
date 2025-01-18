use async_trait::async_trait;
use crate::commons::instrumentation::statsd_config::{StatsdClientGetter, StatsdSvc};

#[async_trait]
pub trait StatsdHelper {
    async fn increment(&self, metric: &str) -> ();
    async fn timing(&self, metric: &str, time: f64) -> ();
}

#[async_trait]
impl StatsdHelper for StatsdSvc {
    async fn increment(&self, metric: &str) -> () {
        let client = self.get_client();
        client.incr(metric)
    }

    async fn timing(&self, metric: &str, time: f64) -> () {
        let client = self.get_client();
        client.timer(metric, time)
    }
}
