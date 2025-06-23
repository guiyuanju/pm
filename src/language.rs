use serde::{Deserialize, Serialize};

use crate::Project;
use crate::template::Template;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
    pub name: String,
    setup_command: String,
}

impl Language {
    pub fn new(name: &str, setup: String) -> Self {
        Language {
            name: name.to_string(),
            setup_command: setup,
        }
    }

    pub fn create_project(&self, project: Project, template: &Template) -> Project {
        // prepare parent dir
        let category = Path::new(&project.category);
        fs::create_dir_all(&category)
            .unwrap_or_else(|e| eprintln!("INFO: failed to create dir {:?}: {}", category, e));

        // replace variable in cmd
        let cmd = project.replace(&self.setup_command, template);

        // execute cmd
        let output = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .current_dir(category)
            .output()
            .expect(&format!(
                "ERROR: failed to execute command '{:?}' in {:?}",
                cmd, category
            ));
        let err = String::from_utf8(output.stderr).expect("ERROR: failed to convert stderr");
        let out = String::from_utf8(output.stdout).expect("failed to convert stdout");
        if err.len() > 0 {
            println!("INFO: STDERR: {err}");
        }
        if out.len() > 0 {
            println!("INFO: STDOUT: {out}");
        }
        if !output.status.success() {
            eprint!("cmd '{cmd}' exit with non-zero status");
            std::process::exit(1);
        }

        // save pm.json file
        fs::write(
            Path::new(&project.path()).join("pm.json"),
            serde_json::to_string_pretty(&project).unwrap(),
        )
        .expect("ERROR: failed to write pm file");

        return project;
    }
}
