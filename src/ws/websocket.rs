use futures::{Stream, TryStream};
use tokio_stream::StreamExt;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tungstenite::http::Request;
use tungstenite::protocol::WebSocketConfig;
use url::Url;

use crate::error::ApplicationError;
use crate::http::user_agent;
use crate::ws::{Heartbeat, Message, Packet, Transport, WebSocketStream};

pub trait RequestBuilderExt {
    fn auth(self, token: &str) -> Self;
    fn agent(self, protocol: &str) -> Self;
    fn subprotocol(self, agent: &str) -> Self;
}

impl RequestBuilderExt for hyper::http::request::Builder {
    fn auth(self, token: &str) -> Self {
        let value = format!("Bearer {}", token);
        self.header(tungstenite::http::header::AUTHORIZATION, value)
    }

    fn agent(self, agent: &str) -> Self {
        self.header(tungstenite::http::header::USER_AGENT, agent)
    }

    fn subprotocol(self, protocol: &str) -> Self {
        self.header(tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL, protocol)
    }
}

pub struct WebSocket {
    inner: WebSocketStream,
    transport: Transport,
    heartbeat: Heartbeat,
}

pub struct WebSocketRequest {
    url: String,
    token: String,
}

impl WebSocketRequest {
    pub fn new(
        url: impl AsRef<str>,
        token: impl AsRef<str>,
    ) -> Result<Request<()>, ApplicationError> {
        let uri = url.as_ref();
        let token = token.as_ref();
        let agent = user_agent();
        let request = Request::builder()
            .uri(uri)
            .auth(token)
            .agent(&agent)
            .subprotocol("tracer")
            .body(())?;
        Ok(request)
    }
}

impl WebSocket {
    pub async fn new(request: Request<()>) -> Result<Self, ApplicationError> {
        info!("Connecting to websocket");

        let (stream, _) = connect_async(request).await?;
        let timeout_in_secs = 2;
        let inner = WebSocketStream::new(stream);
        let heartbeat = Heartbeat::new(timeout_in_secs);
        let transport = Transport::new();

        Ok(Self {
            inner,
            transport,
            heartbeat,
        })
    }

    /// get the next message received on the websocket
    pub async fn next(&mut self) -> Result<Option<Packet>, ApplicationError> {
        tokio::select! {
            Some(_) = self.heartbeat.next() => Ok(Some(Packet::Ping)),
            Some(Ok(Message::Text(message))) = self.inner.next() => {
                let decoded = self.transport.decode(&message);
                Ok(decoded)
            }  else => Ok(None)
        }
    }

    /// send a packet
    pub async fn send(&mut self, packet: Packet) -> Result<(), ApplicationError> {
        debug!("sending packet: {}", packet);
        if let Some(data) = self.transport.encode(packet) {
            self.inner.send(Message::text(data)).await?
        }
        Ok(())
    }

    /// send a ping request
    pub async fn ping(&mut self) -> Result<(), ApplicationError> {
        self.send(Packet::Ping).await
    }

    /// Gracefully close this WebSocket.
    pub async fn close(&mut self) -> Result<(), ApplicationError> {
        self.inner.close().await?;
        self.heartbeat.close();
        Ok(())
    }
}
