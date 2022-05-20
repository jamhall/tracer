use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Command {
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommandCreate {
    name: String,
}

impl Command {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
