use std::time::Duration;

use etcd_client::{Client, ConnectOptions};

pub struct ServiceDiscovery {
    client: Client,
}

impl ServiceDiscovery {
    pub async fn new(endpoints: &[&str], timeout: u64) -> ServiceDiscovery {
        let client = Client::connect(
            endpoints,
            Some(ConnectOptions::new().with_connect_timeout(Duration::new(timeout, 0))),
        )
        .await
        .expect("Failed to connect etcd");
        Self { client }
    }
}
