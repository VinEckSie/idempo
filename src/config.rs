use std::time::Duration;

#[derive(Debug, Clone)]
pub struct IdempoConfig {
    pub ttl: Duration,
    pub require_key: bool,
}

impl Default for IdempoConfig {
    fn default() -> Self {
        Self {
            ttl: Duration::from_secs(60 * 60),
            require_key: true,
        }
    }
}
