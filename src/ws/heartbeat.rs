use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use futures_util::Stream;
use tokio::time::{Instant, Interval};

pub struct Heartbeat {
    timeout: u64,
    last_heartbeat_at: Instant,
    inner: Interval,
    closed: bool,
}

impl Heartbeat {
    pub fn new(timeout: u64) -> Self {
        let interval = tokio::time::interval(Duration::from_millis(1));
        Self {
            timeout,
            last_heartbeat_at: Instant::now(),
            inner: interval,
            closed: false,
        }
    }

    pub fn close(&mut self) {
        self.closed = true
    }
}

impl Stream for Heartbeat {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.closed {
            return Poll::Ready(None);
        }
        match self.inner.poll_tick(cx) {
            Poll::Ready(instance) => {
                if self.last_heartbeat_at.elapsed().as_secs() > self.timeout {
                    self.last_heartbeat_at = instance;
                    return Poll::Ready(Some(()));
                }
                Poll::Pending
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
