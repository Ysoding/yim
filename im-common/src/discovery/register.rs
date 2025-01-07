use super::EndpointInfo;
use anyhow::{Context, Result};
use etcd_client::{Client, ConnectOptions, LeaseKeepAliveStream, PutOptions};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

pub struct Register {
    client: Client,
    key: String,
    value: String,
    lease_id: i64,
    keep_alive: Arc<Mutex<LeaseKeepAliveStream>>,
}

impl Register {
    pub async fn new(
        endpoints: &[&str],
        timeout: u64,
        key: &str,
        endpoint_info: &EndpointInfo,
        lease_ttl: i64,
    ) -> Result<Self> {
        let mut client = Client::connect(
            endpoints,
            Some(ConnectOptions::new().with_connect_timeout(Duration::new(timeout, 0))),
        )
        .await
        .context("Failed to connect to etcd")?;

        let data = endpoint_info.serialize()?;
        let value = String::from_utf8(data)?;

        let lease_grant = client.lease_grant(lease_ttl, None).await?;
        let lease_id = lease_grant.id();

        // key 绑定到 lease，生命周期一致
        client
            .put(
                key,
                value.clone(),
                Some(PutOptions::new().with_lease(lease_id)),
            )
            .await?;

        // keep alive
        let (mut keeper, stream) = client.lease_keep_alive(lease_id).await?;
        keeper.keep_alive().await?;

        Ok(Self {
            client,
            key: key.to_string(),
            value,
            lease_id,
            keep_alive: Arc::new(Mutex::new(stream)),
        })
    }

    pub fn listen_lease_keep_alive(&self) {
        let keep_alive = Arc::clone(&self.keep_alive);
        let lease_id = self.lease_id;
        let key = self.key.clone();
        let value = self.value.clone();

        tokio::spawn(async move {
            while let Some(resp) = keep_alive.lock().await.message().await.unwrap() {
                log::info!(
                    "lease success: leaseId={}, key={}, val={}, resp={:?}",
                    lease_id,
                    key,
                    value,
                    resp
                );
            }
        });
    }

    pub async fn update_value(&mut self, endpoint_info: &EndpointInfo) -> Result<()> {
        let data = endpoint_info.serialize()?;
        let value = String::from_utf8(data)?;

        self.client
            .put(
                self.key.as_str(),
                value.clone(),
                Some(PutOptions::new().with_lease(self.lease_id)),
            )
            .await?;

        self.value = value;

        log::info!(
            "register UpdateValue: leaseId={}, key={}, val={}",
            self.lease_id,
            self.key,
            self.value
        );

        Ok(())
    }

    pub async fn close(&mut self) -> Result<()> {
        self.client.lease_revoke(self.lease_id).await?;
        log::info!(
            "lease close: leaseId={}, key={}, val={}",
            self.lease_id,
            self.key,
            self.value
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]

    async fn basic_test() {
        let endpoints = ["localhost:2379"];

        let endpoint_info = EndpointInfo {
            ip: "127.0.0.1".to_string(),
            port: "6969".to_string(),
            metadata: None,
        };

        let mut reg = Register::new(&endpoints, 10, "/web/node1", &endpoint_info, 5000)
            .await
            .unwrap();

        reg.listen_lease_keep_alive();

        tokio::time::sleep(Duration::from_secs(5)).await;

        let new_endpoint_info = EndpointInfo {
            ip: "127.0.0.1".to_string(),
            port: "6970".to_string(),
            metadata: None,
        };
        reg.update_value(&new_endpoint_info).await.unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;

        reg.close().await.unwrap();
    }
}
