# pm -- An Code Project Manager Written in Rust

A small project manager tool that helps you query and create projects based on custom template quickly.

```sh
Usage: pm <command>

<command>:
project                  | p                 show all projects
template                 | t                 show templates
open <name>              | o                 open a project
new  <name> <template>   | n                 create a project based on a template
```

## Installation

`git clone`, and `cargo build --release`, use `cargo install --path .` to install globally.

## Quick Start

1. Create a file `~/.config/pm/config.json`.
2. Fill the file with the `template.config.json` provided in repo.
3. Done. You can start to use `pm`.

## Config Explanation

- `editors` is the editor you want to use, `open_command` is used to open a directory after creating a project, e.g. `code .`
- `languages` is the programming langage you use, `setup_command` is a series of shell command, which is passed to `sh -s`, you should combine commands with `&&`.
  - `{name}` is the name of your new project
  - `{content}` is the `content` field in corresponding template
- `templates` combines editor and language, you mainly use a predefined template to create a new project.
  - `category` corresponding to the directory you want new projects to be created in.

## Examples

```sh
> pm t
nannou
leetcode_go
leetcode_python

> pm p
[0] 2405 => .../leetcode/python/2405

> pm new test_project nannou
INFO: STDERR:     Creating binary (application) `test_project` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

# and it will open the project in your editor

> pm p | grep test
[0] test_project => .../generative_arts/test_project

# search and open matched project
> pm p | grep test | awk '{print $2}' | xargs pm open
```
