use crate::common::ApplicationConfig;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

use crate::error::ApplicationError;

#[allow(dead_code)]
pub fn parse() -> Result<ApplicationConfig, ApplicationError> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(clap::AppSettings::TrailingVarArg)
        .arg(
            Arg::new("name")
                .short('u')
                .long("name")
                .help("A unique command name")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("command")
                .help("The command to run")
                .required(true)
                .takes_value(true)
                .allow_hyphen_values(true)
                .multiple_values(true)
                .multiple_occurrences(true)
                .help_heading("command"),
        )
        .get_matches();
    let name = matches.value_of("name").unwrap_or("jumping_jacks");
    if let Some(command) = matches.values_of("command") {
        let arguments: Vec<&str> = command.collect();
        let executable = arguments.join(" ");
        info!("executable: {}", executable);
        return Ok(ApplicationConfig::new(executable.as_str(), name));
    }
    Err(ApplicationError::command("Could not parse command"))
}
