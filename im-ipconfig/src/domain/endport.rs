use std::sync::{
    atomic::{AtomicPtr, Ordering},
    Arc,
};

use tokio::sync::{mpsc, Mutex};

use super::{stat::Stat, window::StateWindow};

#[derive(Debug)]
pub(crate) struct Endport {
    ip: String,
    port: String,
    active_score: f64,
    static_score: f64,
    stat: Arc<AtomicPtr<Stat>>,
    window: Arc<Mutex<StateWindow>>,
    stat_ch: mpsc::Sender<Stat>,
}

impl Endport {
    pub(crate) fn new(ip: String, port: String) -> Self {
        let (sender, mut receiver) = mpsc::channel(100);
        let window = Arc::new(Mutex::new(StateWindow::new()));
        let stat = Arc::new(AtomicPtr::new(Box::into_raw(Box::new(Stat::default()))));

        let window_clone = window.clone();
        let stat_clone = stat.clone();
        tokio::spawn(async move {
            while let Some(stat) = receiver.recv().await {
                let mut window = window_clone.lock().await;
                window.add_stat(stat);
                let new_stat = window.get_stat();
                let new_stat_ptr = Box::into_raw(Box::new(new_stat));
                let old_stat_ptr = stat_clone.swap(new_stat_ptr, Ordering::SeqCst);
                unsafe {
                    let _ = Box::from_raw(old_stat_ptr); // Drop the old stat
                }
            }
        });

        Self {
            ip,
            port,
            active_score: 0.0,
            static_score: 0.0,
            stat,
            window,
            stat_ch: sender,
        }
    }

    pub(crate) async fn update_stat(&self, stat: Stat) {
        if let Err(e) = self.stat_ch.send(stat).await {
            log::warn!("update_stat error: {}", e);
        }
    }

    pub fn calculate_score(&mut self) {
        let stat_ptr = self.stat.load(Ordering::SeqCst);
        if !stat_ptr.is_null() {
            let stat = unsafe { &*stat_ptr };
            self.active_score = stat.active_score();
            self.static_score = stat.static_score();
        }
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

        ep.calculate_score();
        assert_relative_eq!(ep.active_score, 1.86);
        assert_relative_eq!(ep.static_score, 6.0);
    }
}
