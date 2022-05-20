use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "packet", content = "content")]
pub enum Packet {
    // send ping packet
    #[serde(rename = "ping")]
    Ping,
    // receive pong packet with a reconnect token
    #[serde(rename = "pong")]
    Pong,
    //  send authentication packet
    #[serde(rename = "authenticate")]
    Authenticate(String),
    //  received authentication successful
    #[serde(rename = "authentication_successful")]
    AuthenticationSuccessful,
    //  received authentication failed
    #[serde(rename = "authentication_failed")]
    AuthenticationFailed(String),
    //  send command output i.e. stdout, stderr
    #[serde(rename = "command_output")]
    CommandOutput { stream: String, message: String },
    //  send command terminated
    #[serde(rename = "command_terminated")]
    CommandTerminated(u32),
    //  receive command terminate packet
    #[serde(rename = "command_terminate")]
    CommandTerminate,
    //  send command launched
    #[serde(rename = "command_launched")]
    CommandLaunched,
}

impl fmt::Display for Packet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Packet::Ping => write!(formatter, "ping"),
            Packet::Pong => write!(formatter, "pong"),
            Packet::Authenticate(_) => write!(formatter, "authenticate"),
            Packet::AuthenticationSuccessful => write!(formatter, "authentication successful"),
            Packet::AuthenticationFailed(message) => {
                write!(formatter, "authenticate failed (message = {})", message)
            }
            Packet::CommandOutput { stream, message } => {
                write!(
                    formatter,
                    "command output ( stream = {}, message = {})",
                    stream, message
                )
            }
            Packet::CommandTerminated(code) => {
                write!(formatter, "command terminated (code = {})", code)
            }
            Packet::CommandTerminate => write!(formatter, "command terminate"),
            Packet::CommandLaunched => write!(formatter, "command launched"),
        }
    }
}
