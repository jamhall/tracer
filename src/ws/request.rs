use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamType {
    #[serde(rename = "stderr")]
    Stderr = 1,
    #[serde(rename = "stdout")]
    Stdout,
    #[serde(rename = "stdin")]
    Stdin,
}

impl fmt::Display for StreamType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            StreamType::Stderr => write!(formatter, "Stderr"),
            StreamType::Stdout => write!(formatter, "Stdout"),
            StreamType::Stdin => write!(formatter, "Stdin"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "request", content = "content")]
pub enum Request {
    #[serde(rename = "event")]
    Event { stream: StreamType, message: String },
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "authenticate")]
    Authenticate(String),
}

impl fmt::Display for Request {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Event { stream, message } => write!(formatter, "Event ({}): {}", stream, message),
            Self::Ping => write!(formatter, "Ping"),
            Self::Authenticate(_) => write!(formatter, "Authenticate"),
        }
    }
}
