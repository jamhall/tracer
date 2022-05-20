use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::ApplicationError;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Configuration {
    environment: Environment,
}

impl FromStr for Configuration {
    type Err = ApplicationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let configuration = serde_yaml::from_str::<Configuration>(s)?;
        Ok(configuration)
    }
}

impl Configuration {
    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    #[cfg(test)]
    pub fn from_str(s: &str) -> Result<Self, ApplicationError> {
        let configuration = serde_yaml::from_str::<Configuration>(s)?;
        Ok(configuration)
    }

    pub fn from_path(path: &Path) -> Result<Self, ApplicationError> {
        let file =
            File::open(path).map_err(|_| ApplicationError::configuration("Unable to open file"))?;
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).map_err(|e| e.into())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Environment {
    host: String,
    https: bool,
    token: String,
    logging: String,
}

impl Environment {
    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn ws_url(&self) -> String {
        if self.https {
            return format!("wss://{}", self.host);
        }
        format!("ws://{}", self.host)
    }

    pub fn api_url(&self) -> String {
        if self.https {
            return format!("https://{}/api", self.host);
        }
        format!("http://{}/api", self.host)
    }

    pub fn logging(&self) -> &str {
        &self.logging
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode() {
        let yaml = r#"
            environment:
                host: localhost:8080
                https: false
                token: a super long token
                logging: DEBUG
        "#;

        let configuration = Configuration {
            environment: Environment {
                host: "localhost:8080".to_string(),
                https: false,
                token: "a super long token".to_string(),
                logging: "DEBUG".to_string(),
            },
        };

        let deserialised_configuration: Configuration = Configuration::from_str(yaml).unwrap();

        assert_eq!(configuration, deserialised_configuration);
    }
}
