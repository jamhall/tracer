use std::str::FromStr;

use crate::error::ApplicationError;
use crate::services::Service;

pub struct Logging {
    level: String,
}

impl Logging {
    pub fn new(level: &str) -> Self {
        Self {
            level: level.into(),
        }
    }
}

impl Service for Logging {
    fn bootstrap(&self) -> Result<(), ApplicationError> {
        let level = self.level.as_str();
        let level = log::LevelFilter::from_str(level)?;
        let mut builder = env_logger::Builder::new();
        builder.filter_level(level);
        builder.init();
        debug!("Initialised logging level to {}", level);
        Ok(())
    }
}
