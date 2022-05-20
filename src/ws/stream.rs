use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Stream;
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use futures_util::Sink;
use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;

use crate::error::ApplicationError;
use crate::ws::Message;

pub struct WebSocketStream {
    inner: tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocketStream {
    pub fn new(inner: tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { inner }
    }

    pub async fn recv(&mut self) -> Option<Result<Message, ApplicationError>> {
        self.next().await
    }

    pub async fn send(&mut self, message: Message) -> Result<(), ApplicationError> {
        self.inner
            .send(message.into())
            .await
            .map_err(|error| error.into())
    }

    /// Gracefully close this WebSocket.
    pub async fn close(&mut self) -> Result<(), ApplicationError> {
        self.inner.close(None).await.map_err(|error| error.into())
    }
}

impl Stream for WebSocketStream {
    type Item = Result<Message, ApplicationError>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner
            .poll_next_unpin(context)
            .map_err(|error| error.into())
            .map_ok(|message| message.into())
    }
}

impl Sink<Message> for WebSocketStream {
    type Error = ApplicationError;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner
            .poll_ready_unpin(cx)
            .map_err(|error| error.into())
    }

    fn start_send(mut self: Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        self.inner
            .start_send_unpin(item.into())
            .map_err(|error| error.into())
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner
            .poll_flush_unpin(cx)
            .map_err(|error| error.into())
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner
            .poll_close_unpin(cx)
            .map_err(|error| error.into())
    }
}
