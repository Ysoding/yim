use serde::Serialize;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use super::{stat::Stat, window::StateWindow};

#[derive(Debug, Serialize, Clone)]
pub(crate) struct Endport {
    ip: String,
    port: String,
    pub(crate) active_score: f64,
    pub(crate) static_score: f64,
    #[serde(skip_serializing)]
    stat: Arc<Mutex<Stat>>,
    #[serde(skip_serializing)]
    _window: Arc<Mutex<StateWindow>>,
    #[serde(skip_serializing)]
    stat_ch: Arc<mpsc::Sender<Stat>>,
}

impl Endport {
    pub(crate) fn new(ip: String, port: String) -> Self {
        let (sender, mut receiver) = mpsc::channel(100);
        let window = Arc::new(Mutex::new(StateWindow::new()));
        let stat = Arc::new(Mutex::new(Stat::default()));

        let window_clone = Arc::clone(&window);
        let stat_clone = Arc::clone(&stat);
        tokio::spawn(async move {
            while let Some(stat) = receiver.recv().await {
                let mut window = window_clone.lock().await;
                window.add_stat(stat);
                let new_stat = window.get_stat();
                *stat_clone.lock().await = new_stat;
            }
        });

        Self {
            ip,
            port,
            active_score: 0.0,
            static_score: 0.0,
            stat,
            _window: window,
            stat_ch: Arc::new(sender),
        }
    }

    pub(crate) async fn update_stat(&self, stat: Stat) {
        if let Err(e) = self.stat_ch.send(stat).await {
            log::warn!("update_stat error: {}", e);
        }
    }

    pub(crate) async fn update_score(&mut self) {
        let stat = self.stat.lock().await;
        self.active_score = stat.active_score();
        self.static_score = stat.static_score();
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use approx::assert_relative_eq;

    use super::*;

    #[tokio::test]
    async fn test_basic() {
        let mut ep = Endport::new("localhost".to_string(), "6969".to_string());

        ep.update_stat(Stat::new(10.0, 5_000_000_000.0)).await;
        ep.update_stat(Stat::new(20.0, 5_000_000_000.0)).await;

        tokio::time::sleep(Duration::from_secs(1)).await;

        ep.update_score().await;
        assert_relative_eq!(ep.active_score, 1.86);
        assert_relative_eq!(ep.static_score, 6.0);
    }
}
