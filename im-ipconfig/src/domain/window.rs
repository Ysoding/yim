use tokio::sync::mpsc;

use super::stat::Stat;

const WINDOW_SIZE: i32 = 5;

#[derive(Debug)]
pub(crate) struct StateWindow {
    state_queue: Vec<Stat>,
    sum_stat: Stat,
    idx: usize,
}

impl StateWindow {
    pub(crate) fn new() -> Self {
        Self {
            state_queue: Vec::with_capacity(WINDOW_SIZE as usize),
            sum_stat: Stat::default(),
            idx: 0,
        }
    }

    pub(crate) fn get_stat(&self) -> Stat {
        let mut stat = self.sum_stat.clone();
        stat.avg(f64::from(WINDOW_SIZE));
        stat
    }

    pub(crate) fn add_stat(&mut self, stat: Stat) {
        let idx = self.idx % (WINDOW_SIZE as usize);

        if let Some(a) = self.state_queue.get(idx) {
            self.sum_stat.sub(a);
        }

        self.sum_stat.add(&stat);

        self.state_queue.insert(idx, stat);
        self.idx += 1;
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_basic() {
        let mut state_window = StateWindow::new();

        state_window.add_stat(Stat::new(10.0, 100.0));
        state_window.add_stat(Stat::new(20.0, 200.0));
        state_window.add_stat(Stat::new(30.0, 300.0));
        state_window.add_stat(Stat::new(40.0, 400.0));
        state_window.add_stat(Stat::new(50.0, 500.0));

        let avg_stat = state_window.get_stat();
        assert_relative_eq!(avg_stat.connect_num, 30.0);
        assert_relative_eq!(avg_stat.message_bytes, 300.0);

        state_window.add_stat(Stat::new(60.0, 600.0));

        let avg_stat = state_window.get_stat();
        assert_relative_eq!(avg_stat.connect_num, 40.0);
        assert_relative_eq!(avg_stat.message_bytes, 400.0);
    }
}
