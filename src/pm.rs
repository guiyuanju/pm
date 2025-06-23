use crate::editor::Editor;
use crate::language::Language;
use crate::template::Template;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct PM {
    pub editors: Vec<Editor>,
    pub languages: Vec<Language>,
    pub templates: Vec<Template>,
}

impl PM {
    pub fn new(editors: Vec<Editor>, languages: Vec<Language>, templates: Vec<Template>) -> Self {
        Self {
            editors,
            languages,
            templates,
        }
    }

    pub fn path() -> String {
        let home = env::home_dir().expect("ERROR: failed to get home dir");
        let home = home
            .to_str()
            .expect("ERROR: failed to get str from home dir");
        let mut config_dir = Path::new(home).join(".config");
        if !config_dir.exists() {
            config_dir = config_dir
                .parent()
                .expect("ERROR: failed to get parent dir")
                .to_path_buf();
            return config_dir
                .join(".pm_config.json")
                .to_str()
                .unwrap()
                .to_string();
        }

        config_dir
            .join("pm")
            .join("config.json")
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn template(&self, name: &str) -> Option<&Template> {
        self.templates.iter().find(|t| t.name == name)
    }
}
