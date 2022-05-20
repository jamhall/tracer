use std::rc::Rc;
use std::str::FromStr;

use tokio::sync::mpsc::UnboundedReceiver;

use crate::cmd::Command;
use crate::config::configuration::Environment;
use crate::config::Configuration;
use crate::error::ApplicationError;
use crate::manager::Manager;
use crate::services::{Logging, Service};

pub struct Application {
    services: Vec<Box<dyn Service>>,
    environment: Environment,
    shutdown: UnboundedReceiver<()>,
}

#[allow(dead_code)]
impl Application {
    pub fn new(environment: Environment, shutdown: UnboundedReceiver<()>) -> Self {
        let logging = Logging::new(environment.logging());

        Self {
            environment,
            shutdown,
            services: vec![Box::new(logging)],
        }
    }

    /// start the application for a given command
    pub async fn run(&self, command: Command) -> Result<(), ApplicationError> {
        for service in &self.services {
            service.bootstrap()?;
        }
        let manager = Manager::new(&self.environment);
        manager.spawn(command).await
    }

    pub async fn shutdown(&self) -> Result<(), ApplicationError> {
        todo!()
    }
}
