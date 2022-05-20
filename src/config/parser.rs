use crate::config::Configuration;
use std::env;
use std::path::PathBuf;

use crate::error::ApplicationError;

#[derive(Default)]
pub struct ConfigurationParser;

impl ConfigurationParser {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse(&self) -> Result<Configuration, ApplicationError> {
        return match self.find_configuration_file() {
            Some(file) => {
                let path = file.as_path();
                let configuration = Configuration::from_path(path)?;
                Ok(configuration)
            }
            None => Err(ApplicationError::configuration(
                "Could not find a configuration file",
            )),
        };
    }

    /// Get the location of the first found default config file paths
    /// according to the following order:
    ///
    /// 1. $XDG_CONFIG_HOME/tracer/tracer.yml
    /// 2. $XDG_CONFIG_HOME/tracer.yml
    /// 3. $HOME/.config/tracer/tracer.yml
    /// 4. $HOME/.tracer.yml
    #[cfg(not(windows))]
    pub fn find_configuration_file(&self) -> Option<PathBuf> {
        // Try using XDG location by default.
        xdg::BaseDirectories::with_prefix("tracer")
            .ok()
            .and_then(|xdg| xdg.find_config_file("tracer.yml"))
            .or_else(|| {
                xdg::BaseDirectories::new()
                    .ok()
                    .and_then(|fallback| fallback.find_config_file("tracer.yml"))
            })
            .or_else(|| {
                if let Ok(home) = env::var("HOME") {
                    let fallback = PathBuf::from(&home).join(".config/tracer/tracer.yml");
                    if fallback.exists() {
                        return Some(fallback);
                    }
                    let fallback = PathBuf::from(&home).join(".tracer.yml");
                    if fallback.exists() {
                        return Some(fallback);
                    }
                }
                None
            })
    }

    #[cfg(windows)]
    pub fn find_configuration_file(&self) -> Option<PathBuf> {
        dirs::config_dir()
            .map(|path| path.join("tracer\\tracer.yml"))
            .filter(|new| new.exists())
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Configuration;

    use super::*;

    #[test]
    fn config_read_eof() {
        let parser = ConfigurationParser::new();
        let configuration = parser.parse().unwrap();
        let host = configuration.environment().host();
        println!("host = {:?}", host);
    }
}
