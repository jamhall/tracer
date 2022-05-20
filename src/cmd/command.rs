use std::os::unix::prelude::ExitStatusExt;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    process::{Command as StdCommand, Stdio},
    sync::Arc,
};

use os_pipe::{pipe, PipeReader, PipeWriter};
use shared_child::SharedChild;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, RwLock};

use crate::cmd::child::CommandChild;
use crate::cmd::event::CommandEvent;
use crate::error::ApplicationError;

macro_rules! get_std_command {
    ($self: ident) => {{
        let mut command = StdCommand::new($self.executable);
        command.args(&$self.args);
        command.stdout(Stdio::piped());
        command.stdin(Stdio::piped());
        command.stderr(Stdio::piped());
        command.envs($self.env);
        #[cfg(windows)]
        command.creation_flags(CREATE_NO_WINDOW);
        command
    }};
}

async fn handle_stdout(reader: PipeReader, lock: Arc<RwLock<()>>, tx: Sender<CommandEvent>) {
    lock.read().await;
    let reader = BufReader::new(reader);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                tx.send(CommandEvent::Stdout(line)).await;
            }
            Err(error) => {
                tx.send(CommandEvent::Error(error.to_string())).await;
            }
        };
    }
}

async fn handle_stderr(reader: PipeReader, lock: Arc<RwLock<()>>, tx: Sender<CommandEvent>) {
    lock.read().await;
    let reader = BufReader::new(reader);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                tx.send(CommandEvent::Stderr(line)).await;
            }
            Err(error) => {
                tx.send(CommandEvent::Error(error.to_string())).await;
            }
        };
    }
}

async fn handle_exit(
    child: Arc<SharedChild>,
    lock: Arc<RwLock<()>>,
    tx: Sender<CommandEvent>,
) -> Result<(), SendError<CommandEvent>> {
    match child.wait() {
        Ok(status) => {
            lock.write().await;
            tx.send(CommandEvent::Exited {
                code: status.code(),
                signal: status.signal(),
            })
            .await
        }
        Err(error) => {
            lock.write().await;
            tx.send(CommandEvent::Error(error.to_string())).await
        }
    }
}

/// The type to spawn commands.
#[derive(Debug)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: HashMap<String, String>,
}

impl Command {
    /// Creates a new command for launching the given program.
    pub fn new<S: Into<String>>(executable: S) -> Self {
        Self {
            executable: executable.into(),
            args: Default::default(),
            env: Default::default(),
        }
    }

    /// Appends arguments to the cmd.
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for arg in args {
            self.args.push(arg.as_ref().to_string());
        }
        self
    }

    /// Adds or updates multiple environment variable mappings.
    pub fn envs(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }

    /// Spawns the cmd.
    pub fn spawn(self, bus: Sender<CommandEvent>) -> Result<u32, ApplicationError> {
        let mut command = get_std_command!(self);
        let (stdout_reader, stdout_writer) = pipe()?;
        let (stderr_reader, stderr_writer) = pipe()?;
        let (stdin_reader, stdin_writer) = pipe()?;

        command.stdout(stdout_writer);
        command.stderr(stderr_writer);
        command.stdin(stdin_reader);

        let child = SharedChild::spawn(&mut command)?;
        let child = Arc::new(child);
        let lock = Arc::new(RwLock::new(()));

        let stdout_handler = handle_stdout(stdout_reader, lock.clone(), bus.clone());
        let stderr_handler = handle_stderr(stderr_reader, lock.clone(), bus.clone());
        let exit_handler = handle_exit(child.clone(), lock, bus.clone());

        tokio::spawn(stdout_handler);
        tokio::spawn(stderr_handler);
        tokio::spawn(exit_handler);

        Ok(child.id())
    }
}
