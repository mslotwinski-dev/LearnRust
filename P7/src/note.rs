use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub content: String,
}

impl Note {
    pub fn new(title: String, content: String) -> Note {
        Note { title, content }
    }
}
