use std::{
    fs::{self},
    io::{self, BufReader},
    path::Path,
};

use crate::{Editor, Language, PM, project::Project};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Template {
    pub name: String,
    pub category: String,
    pub editor: String,
    pub language: String,
    pub tags: Vec<String>,
    pub content: String,
}

impl Template {
    pub fn new(
        name: &str,
        category: &str,
        editor: &str,
        language: &str,
        tags: Vec<String>,
        content: String,
    ) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            editor: editor.to_string(),
            language: language.to_string(),
            tags: tags,
            content,
        }
    }

    pub fn create(&self, pm: &PM, project_name: &str) -> Project {
        let language: &Language = pm
            .languages
            .iter()
            .find(|l| l.name == self.language)
            .expect("ERROR: failed to locate language");

        let editor: &Editor = pm
            .editors
            .iter()
            .find(|e| e.name == self.editor)
            .expect("ERROR: failed to locate editor");

        let project = Project::new(
            project_name,
            &self.category,
            self.tags.clone(),
            editor.clone(),
        );

        language.create_project(project, self)
    }

    pub fn scan_projects(&self) -> (Vec<Project>, u32) {
        let mut res: Vec<Project> = vec![];
        let mut count = 0;

        let mut cb = |path: &Path| -> Result<(), String> {
            count += 1;
            let ok = path
                .file_name()
                .ok_or("failed to convert file name")?
                .to_str()
                .ok_or("failed to convert to str")?
                == "pm.json";
            if ok {
                let p: Project = serde_json::from_reader(BufReader::new(
                    fs::File::open(path)
                        .map_err(|e| format!("failed to open {:?}: {:?}", path, e))?,
                ))
                .map_err(|e| format!("failed to deserialize {:?}: {:?}", path, e))?;
                res.push(p);
            }
            return Ok(());
        };
        visit_dirs(Path::new(&self.category), &mut cb).expect("ERROR: failed to scan projects");

        (res, count)
    }
}

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&Path) -> Result<(), String>) -> io::Result<()> {
    if dir.is_file() {
        if let Err(e) = cb(dir) {
            eprintln!("ERROR: failed to visit file {:?}: {e}", dir)
        }
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        visit_dirs(&path, cb)?;
    }

    Ok(())
}
