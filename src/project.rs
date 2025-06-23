use crate::{editor::Editor, template::Template};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub category: String,
    pub tags: Vec<String>,
    pub editor: Editor,
}

impl Project {
    pub fn new(name: &str, category: &str, tags: Vec<String>, editor: Editor) -> Self {
        Self {
            name: name.to_string(),
            editor: editor,
            category: category.to_string(),
            tags,
        }
    }

    pub fn replace(&self, command: &str, template: &Template) -> String {
        command
            .replace("{name}", &self.name)
            .replace("{category}", &self.category)
            .replace("{content}", &template.content)
    }

    pub fn path(&self) -> String {
        Path::new(&self.category)
            .join(&self.name)
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn open(&self) {
        self.editor.open(&self.path());
    }
}
