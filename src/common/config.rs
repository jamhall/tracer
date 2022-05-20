#![allow(dead_code)]
pub struct ApplicationConfig {
    executable: String,
    name: String,
}

impl ApplicationConfig {
    pub fn new(executable: &str, name: &str) -> Self {
        ApplicationConfig {
            executable: executable.into(),
            name: name.into(),
        }
    }

    pub fn executable(&self) -> &str {
        &self.executable
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
