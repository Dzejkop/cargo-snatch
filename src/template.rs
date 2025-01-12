use std::path::Path;

use eyre::ContextCompat;
use handlebars::Handlebars;
use include_dir::{include_dir, Dir, File};
use serde::{Deserialize, Serialize};

const TEMPLATE_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/template_repo");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub name: String,
    pub repo: String,
    pub repo_stub: String,
    pub author: Option<String>,
}

fn files_in<'a>(dir: &Dir<'a>) -> Vec<&'a File<'a>> {
    let mut files = vec![];

    for file in dir.files() {
        files.push(file);
    }

    for dir in dir.dirs() {
        files.extend(files_in(dir));
    }

    files
}

pub fn instance_in(dir: impl AsRef<Path>, context: &Context) -> eyre::Result<()> {
    let dir = dir.as_ref();
    let context = serde_json::to_value(context)?;

    let all_files = files_in(&TEMPLATE_DIR);

    for file in all_files {
        let new_path = dir.join(file.path());

        if let Some(parent) = new_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let template = file.contents_utf8().context("Invalid template content")?;

        let reg = Handlebars::new();
        let rendered = reg.render_template(template, &context)?;

        std::fs::write(new_path, rendered)?;
    }

    Ok(())
}
