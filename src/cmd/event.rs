use std::fmt;

pub enum CommandEvent {
    Stderr(String),
    Stdout(String),
    Error(String),
    Exited {
        code: Option<i32>,
        signal: Option<i32>,
    },
}
