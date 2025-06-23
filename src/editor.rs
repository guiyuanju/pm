use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Editor {
    pub name: String,
    open_command: String,
}

impl Editor {
    pub fn new(name: &str, open_command: &str) -> Self {
        Editor {
            name: name.to_string(),
            open_command: open_command.to_string(),
        }
    }

    pub fn open(&self, path: &str) {
        Command::new(&self.open_command)
            .arg(path)
            .spawn()
            .expect("failed to open editor");
    }
}
