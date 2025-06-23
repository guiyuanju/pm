create, search, open, delete, archive, tag, tree strcuture, template

Entities:
     - Editor
     - Lang
     - Template
     - Project
     - Tag

Use Cases:
1. create editor:
     - name: zed
     - command: zed PATH
2. create language:
     - name : rust
     - setup command: cd DIR; cargo new NAME;
2. create project template for GA:
     - name: nannou_template
     - lang: rust, custom command: cargo add nannou
     - editor: zed
     - storage path
2. create a GA project from template called Ants:
     - select the nannou template
     - provide project name: Ants
     - set tags: ga, rust
     - optional: custom lang/editor/lib/storage path
3. open
4. navigate:
     - navigate by folder tree
     - search by name, tag
5. manage:
     - archive project
     - delete project
     - edit project

User Interface:
     - Core
         - command
         - DSL
     - Mode
         - One-off
         - Server
     - API
         - REST
         - Programming API (built-in for Rust)
         - Command Line command
     - Interactive UI
         - TUI
         - Web
         - Raycast

Architechure:
     - pm.toml
     - folder with pm.toml == project
     - .config/pm/config.toml: global config like editor, template, langues, tags
     - .config/pm/cache: cache previous computed info for performance
