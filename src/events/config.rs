use std::time::Duration;

pub struct Config {
    pub round_floats: bool,
    pub replay_interval: Option<Duration>,
}

impl From<crate::Cli> for Config {
    fn from(value: crate::Cli) -> Self {
        let crate::Cli {
            round_floats,
            float_replay_interval_minutes,
        } = value;
        Self {
            round_floats,
            replay_interval: float_replay_interval_minutes.map(|x| Duration::from_secs(60 * x)),
        }
    }
}
