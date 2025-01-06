use std::{sync::Arc, time::Duration};

use anyhow::{Context, Result};
use etcd_client::{Client, ConnectOptions, EventType, GetOptions, WatchOptions};
use tokio::sync::Mutex;

pub struct Discovery {
    client: Arc<Mutex<Client>>,
}

impl Discovery {
    pub async fn new(endpoints: &[&str], timeout: u64) -> Result<Self> {
        let client = Client::connect(
            endpoints,
            Some(ConnectOptions::new().with_connect_timeout(Duration::new(timeout, 0))),
        )
        .await
        .context("Failed to connect to etcd")?;

        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub async fn watch<F, G>(&self, prefix: &str, set: F, del: G) -> Result<()>
    where
        F: Fn(&str, &str) + Send + Sync + 'static,
        G: Fn(&str, &str) + Send + Sync + 'static,
    {
        let resp = {
            let mut client = self.client.lock().await;
            client
                .get(prefix, Some(GetOptions::new().with_prefix()))
                .await?
        };

        for kv in resp.kvs() {
            set(kv.value_str()?, kv.value_str()?)
        }

        let watch_client = Arc::clone(&self.client);
        let prefix = prefix.to_string();
        tokio::spawn(async move { Self::watcher(watch_client, prefix, &set, &del).await });
        Ok(())
    }

    async fn watcher<F, G>(client: Arc<Mutex<Client>>, prefix: String, set: &F, del: &G)
    where
        F: Fn(&str, &str) + Send + Sync + 'static,
        G: Fn(&str, &str) + Send + Sync + 'static,
    {
        let (mut _watcher, mut stream) = {
            let mut client = client.lock().await;
            client
                .watch(prefix, Some(WatchOptions::new().with_prefix()))
                .await
                .context("Failed to start etcd watch")
                .unwrap()
        };

        while let Some(resp) = stream.message().await.unwrap() {
            for event in resp.events() {
                match event.event_type() {
                    EventType::Put => {
                        let kv = event.kv().unwrap();
                        set(kv.key_str().unwrap(), kv.value_str().unwrap());
                    }
                    EventType::Delete => {
                        let kv = event.kv().unwrap();
                        del(kv.key_str().unwrap(), kv.value_str().unwrap());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn basic_test() {
        let endpoints = ["localhost:2379"];

        let d = Discovery::new(&endpoints, 10).await.unwrap();

        let set = |key: &str, value: &str| {
            println!("set {}={}", key, value);
        };

        let del = |key: &str, value: &str| {
            println!("del {}={}", key, value);
        };

        d.watch("/web/", set, del).await.unwrap();
        d.watch("/aaa/", set, del).await.unwrap();

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
