use futures_util::StreamExt;
use tokio::sync::mpsc::unbounded_channel;

use crate::cmd::Command;
use crate::config::configuration::Environment;
use crate::error::ApplicationError;
use crate::ws::{Packet, WebSocket, WebSocketRequest};

pub struct Manager {
    environment: Environment,
}

impl Manager {
    pub fn new(environment: &Environment) -> Self {
        Self {
            environment: environment.to_owned(),
        }
    }

    pub async fn create_websocket(&self, session_id: &str) -> Result<WebSocket, ApplicationError> {
        let url = format!("{}/ws/sessions/{}", self.environment.ws_url(), session_id);
        let request = WebSocketRequest::new(url, "token")?;
        WebSocket::new(request).await
    }

    pub async fn spawn(&self, command: Command) -> Result<(), ApplicationError> {
        debug!("Spawning manager");
        let (rx, mut tx) = unbounded_channel();
        let mut websocket = self.create_websocket("123-123-123-123").await?;

        rx.send("Hello");
        loop {
            tokio::select! {
                Ok(Some(packet)) = websocket.next() => {
                    // info!("closing websocket");
                    // websocket.close().await;
                    // heartbeat.close();
                    // tx.close();
                    match packet {
                        Packet::Ping => {
                            // send ping back
                            debug!("Sending ping back");
                            websocket.ping().await;
                        }
                        Packet::Pong => {}
                        Packet::AuthenticationSuccessful => {
                            debug!("successfully authenticated");
                            // launch spawn command
                        }
                        Packet::AuthenticationFailed(_) => {
                            // exit program
                        }
                        Packet::CommandTerminate => {
                            // terminate the launched command
                        }
                        _ => debug!("ignoring unknown packet")
                    }
                    debug!("packet received: {}", packet);
                }
                else => {
                    info!("shutting down system");
                    break;
                }
            }
        }
        info!("shut down system");
        Ok(())
    }
}
