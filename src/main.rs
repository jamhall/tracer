#[macro_use]
extern crate log;

use futures::StreamExt;
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

use crate::application::Application;
use crate::cmd::Command;
use crate::config::ConfigurationParser;
use crate::error::ApplicationError;
use crate::services::Logging;

mod application;
mod cli;
mod cmd;
mod common;
mod config;
mod error;
mod http;
mod manager;
mod services;
mod ws;

async fn handle_signals(sender: UnboundedSender<()>, mut signals: Signals) {
    while let Some(signal) = signals.next().await {
        match signal {
            SIGHUP => {
                println!("hello sigup!");
                // Reload configuration
                // Reopen the log file
            }
            SIGTERM | SIGINT | SIGQUIT => {
                println!("Shutting down system");
                sender.send(()).unwrap();
                // Shutdown the system;
            }
            _ => unreachable!(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let parser = ConfigurationParser::new();
    let configuration = parser.parse()?;
    let environment = configuration.environment();
    let (shutdown_send, shutdown_recv) = mpsc::unbounded_channel();
    let command = Command::new("./examples/count.sh").args(&["5"]);
    let application = Application::new(environment.to_owned(), shutdown_recv);

    // handle shutdown signals...
    let signals = Signals::new(&[SIGHUP, SIGTERM, SIGINT, SIGQUIT])?;

    //
    // let handle = signals.handle();
    //
    let signals_task = tokio::spawn(handle_signals(shutdown_send.clone(), signals));
    //
    application.run(command).await?;
    //
    // // info!("Spawning command");
    // // let command = Command::new("./examples/count.sh");
    // // let mut spawned = command.spawn().unwrap();
    // // loop {
    // //     tokio::select! {
    // //         line = spawned.next() => {
    // //             if let Some(line) = line {
    // //                 info!("Got line: {:?}", line);
    // //             }
    // //
    // //         }
    // //
    // //     }
    // // }
    //
    //
    //
    // // let process = Process::new("./examples/count.sh");
    // //
    // // match process.run() {
    // //     Ok(mut receiver) => {
    // //         loop {
    // //             tokio::select! {
    // //                 line = receiver.recv() => {
    // //                     if let Some(line) = line {
    // //                         info!("Got line: {:?}", line);
    // //                     }
    // //                 }
    // //             }
    // //         }
    // //     }
    // //     Err(error) => error!("There was an error: {:?}", error)
    // // }
    //
    // handle.close();
    // signals_task.await?;

    Ok(())
}
