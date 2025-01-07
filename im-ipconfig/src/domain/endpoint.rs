use super::stat::Stat;

pub struct Endpoint {
    ip: String,
    port: String,
    active_score: f64,
    static_score: f64,
    stat: Stat,
}
