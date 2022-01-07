#[derive(Debug, Deserialize)]
pub struct TimerConfig {
    pub base_time: u64,
    pub time_per_turn: u64,
}

#[derive(Debug, Deserialize)]
pub struct Options {
    pub n_players: u8,
    pub variant: super::Variant,
    pub timed: Option<TimerConfig>,
}

impl Options {
    pub fn new(yaml: &str) -> Options {
        serde_yaml::from_str(yaml).expect("Bad yaml")
    }
}
