use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;

use hyper::http::uri::InvalidUri;
use hyper::Error;
use log::ParseLevelError;
use reqwest::header::InvalidHeaderValue;
use tokio::task::JoinError;
use url::ParseError;

#[derive(Clone, Debug)]
pub struct ApplicationError {
    message: String,
    kind: ApplicationErrorKind,
}

impl ApplicationError {
    fn new(message: impl AsRef<str>, kind: ApplicationErrorKind) -> Self {
        Self {
            message: message.as_ref().to_string(),
            kind,
        }
    }

    pub fn transport(explanation: impl AsRef<str>) -> Self {
        Self::new(explanation, ApplicationErrorKind::Transport)
    }

    pub fn command(explanation: impl AsRef<str>) -> Self {
        Self::new(explanation, ApplicationErrorKind::Command)
    }

    pub fn configuration(explanation: impl AsRef<str>) -> Self {
        Self::new(explanation, ApplicationErrorKind::Configuration)
    }

    pub fn io(explanation: impl AsRef<str>) -> Self {
        Self::new(explanation, ApplicationErrorKind::Io)
    }

    pub fn kind(&self) -> &ApplicationErrorKind {
        &self.kind
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}; {}", self.kind, self.message)
    }
}

impl From<serde_yaml::Error> for ApplicationError {
    fn from(error: serde_yaml::Error) -> Self {
        let message = format!("issue with parsing the configuration YAML: {}", error);
        ApplicationError::configuration(message)
    }
}

impl From<serde_json::Error> for ApplicationError {
    fn from(error: serde_json::Error) -> Self {
        let message = format!("issue with parsing JSON: {}", error);
        ApplicationError::io(message)
    }
}

impl From<InvalidUri> for ApplicationError {
    fn from(error: InvalidUri) -> Self {
        let message = format!("invalid URI: {}", error);
        ApplicationError::io(message)
    }
}

impl From<hyper::Error> for ApplicationError {
    fn from(error: Error) -> Self {
        ApplicationError::io(format!("{}", error))
    }
}

impl From<clap::Error> for ApplicationError {
    fn from(error: clap::Error) -> Self {
        let message = format!("issue with parsing the command: {}", error);
        ApplicationError::configuration(message)
    }
}

impl From<InvalidHeaderValue> for ApplicationError {
    fn from(error: InvalidHeaderValue) -> Self {
        ApplicationError::io(format!("{}", error))
    }
}

impl From<reqwest::Error> for ApplicationError {
    fn from(error: reqwest::Error) -> Self {
        ApplicationError::io(format!("{}", error))
    }
}

impl From<tungstenite::Error> for ApplicationError {
    fn from(error: tungstenite::Error) -> Self {
        let message = format!("http issue: {:?}", error);
        ApplicationError::transport(message)
    }
}

impl From<tungstenite::http::Error> for ApplicationError {
    fn from(error: tungstenite::http::Error) -> Self {
        let message = format!("http: {}", error);
        ApplicationError::transport(message)
    }
}

impl From<url::ParseError> for ApplicationError {
    fn from(error: ParseError) -> Self {
        let message = format!("unable to parse URL: {}", error);
        ApplicationError::configuration(message)
    }
}

impl From<tokio::task::JoinError> for ApplicationError {
    fn from(error: JoinError) -> Self {
        let message = format!("unable to join thread: {}", error);
        ApplicationError::transport(message)
    }
}

impl From<std::io::Error> for ApplicationError {
    fn from(error: io::Error) -> Self {
        let message = format!("io issue: {}", error);
        ApplicationError::io(message)
    }
}

impl From<ParseLevelError> for ApplicationError {
    fn from(_: ParseLevelError) -> Self {
        ApplicationError::configuration("Invalid logging level supplied")
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ApplicationErrorKind {
    Io,
    Command,
    Transport,
    Configuration,
}

impl Display for ApplicationErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = match self {
            ApplicationErrorKind::Io => "io",
            ApplicationErrorKind::Command => "command",
            ApplicationErrorKind::Transport => "transport",
            ApplicationErrorKind::Configuration => "configuration issue",
        };
        write!(f, "{}", string)
    }
}
