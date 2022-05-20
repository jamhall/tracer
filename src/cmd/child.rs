use std::io::Write;
use std::sync::Arc;

use os_pipe::PipeWriter;
use shared_child::SharedChild;

use crate::error::ApplicationError;

#[derive(Debug)]
pub struct CommandChild {
    inner: Arc<SharedChild>,
    stdin_writer: PipeWriter,
}

impl CommandChild {
    pub fn new(inner: Arc<SharedChild>, stdin_writer: PipeWriter) -> Self {
        Self {
            inner,
            stdin_writer,
        }
    }
    /// Writes to process stdin.
    pub fn write(&mut self, buf: &[u8]) -> Result<(), ApplicationError> {
        self.stdin_writer.write_all(buf)?;
        Ok(())
    }

    /// Sends a kill signal to the child.
    pub fn kill(self) -> Result<(), ApplicationError> {
        self.inner.kill()?;
        Ok(())
    }

    /// Returns the process pid.
    pub fn pid(&self) -> u32 {
        self.inner.id()
    }
}
