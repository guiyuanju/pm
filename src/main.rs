mod editor;
mod language;
mod pm;
mod project;
mod tag;
mod template;

use editor::Editor;
use language::Language;
use pm::PM;
use project::Project;
use std::{env, fs::File, io::BufReader, process::exit};

fn main() {
    let pm: PM = serde_json::from_reader(BufReader::new(
        File::open(PM::path()).expect("ERROR: failed to open config.json"),
    ))
    .expect("ERROR: failed to deserialze config.json");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        exit(1);
    }

    match args[1].as_str() {
        "project" | "p" => {
            let projects = scan_all_projects(&pm);
            for (i, p) in projects.iter().enumerate() {
                println!("[{}] {} => {}/{}", i, p.name, p.category, p.name);
            }
        }
        "template" | "t" => {
            for t in &pm.templates {
                println!("{}", t.name);
            }
        }
        "open" | "o" => {
            if args.len() < 3 {
                print_help();
                exit(1);
            }
            let name = &args[2];
            let projects = scan_all_projects(&pm);
            let project = projects
                .iter()
                .find(|p| &p.name == name)
                .unwrap_or_else(|| {
                    eprint!("ERROR: failed to locate project {}", name);
                    exit(1);
                });
            project.open();
        }
        "new" | "n" => {
            if args.len() < 4 {
                print_help();
                exit(1);
            }
            let name = &args[2];
            let template = &args[3];
            pm.template(&template)
                .expect("ERROR: failed to locate template")
                .create(&pm, &name)
                .open();
        }
        _ => {}
    }
}

fn scan_all_projects(pm: &PM) -> Vec<Project> {
    let mut count = 0;
    let mut projects: Vec<Project> = vec![];
    for t in &pm.templates {
        let (mut ps, c) = t.scan_projects();
        count += c;
        projects.append(&mut ps);
    }
    // println!("INFO: scanned {} files", count);
    return projects;
}

fn print_help() {
    let msg = r#"Usage: pm <command>

<command>:
project                  | p                 show all projects
template                 | t                 show templates
open <name>              | o                 open a project
new  <name> <template>   | n                 create a project based on a template
"#;
    eprint!("{}", msg);
}
