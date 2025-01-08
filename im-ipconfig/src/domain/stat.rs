#[derive(Default, Clone, Debug)]
pub(crate) struct Stat {
    pub(crate) connect_num: f64,   // 总体持有的长连接数量 的剩余值
    pub(crate) message_bytes: f64, // 每秒收发消息的总字节数 的剩余值
}

impl Stat {
    #[allow(dead_code)]
    pub(crate) fn new(connect_num: f64, message_bytes: f64) -> Self {
        Stat {
            connect_num,
            message_bytes,
        }
    }

    pub(crate) fn active_score(&self) -> f64 {
        convert_to_gb(self.message_bytes)
    }

    pub(crate) fn static_score(&self) -> f64 {
        self.connect_num
    }

    pub(crate) fn avg(&mut self, num: f64) {
        self.connect_num /= num;
        self.message_bytes /= num;
    }

    pub(crate) fn add(&mut self, other: &Stat) {
        self.connect_num += other.connect_num;
        self.message_bytes += other.message_bytes;
    }

    pub(crate) fn sub(&mut self, other: &Stat) {
        self.connect_num -= other.connect_num;
        self.message_bytes -= other.message_bytes;
    }
}

fn convert_to_gb(bytes: f64) -> f64 {
    round_to_two_decimals(bytes / f64::from(1 << 30))
}

fn round_to_two_decimals(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_calculate_active_score() {
        let stat = Stat::new(100.0, 5_000_000_000.0); // 5 GB
        assert_relative_eq!(stat.active_score(), 4.66); // 5 GB in GB, rounded to two decimal places
    }

    #[test]
    fn test_calculate_static_score() {
        let stat = Stat::new(100.0, 5_000_000_000.0);
        assert_relative_eq!(stat.static_score(), 100.0);
    }

    #[test]
    fn test_avg() {
        let mut stat = Stat::new(100.0, 5_000_000_000.0);
        stat.avg(2.0);
        assert_relative_eq!(stat.connect_num, 50.0);
        assert_relative_eq!(stat.message_bytes, 2_500_000_000.0);
    }

    #[test]
    fn test_add() {
        let mut stat1 = Stat::new(100.0, 5_000_000_000.0);
        let stat2 = Stat::new(50.0, 2_500_000_000.0);
        stat1.add(&stat2);
        assert_relative_eq!(stat1.connect_num, 150.0);
        assert_relative_eq!(stat1.message_bytes, 7_500_000_000.0);
    }

    #[test]
    fn test_sub() {
        let mut stat1 = Stat::new(100.0, 5_000_000_000.0);
        let stat2 = Stat::new(50.0, 2_500_000_000.0);
        stat1.sub(&stat2);
        assert_relative_eq!(stat1.connect_num, 50.0);
        assert_relative_eq!(stat1.message_bytes, 2_500_000_000.0);
    }
}
