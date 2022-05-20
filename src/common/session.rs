use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    id: String,
    token: String,
}

impl Session {
    pub fn new(id: String, token: String) -> Self {
        Self { id, token }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}
