use std::{
    path::{Path, PathBuf},
    process::Command,
};

use clap::Parser;
use config::Config;
use eyre::eyre;
use reqwest::StatusCode;
use template::Context;

pub mod config;
pub mod template;
pub mod types;

#[derive(Debug, Parser, Clone)]
pub struct Opt {
    /// Name of the crate to snatch
    name: String,
}

pub fn ensure_deps() {
    if let Err(_err) = which::which("gh") {
        eprintln!("`gh` is missing, visit https://cli.github.com/ to install it");

        std::process::exit(1);
    }

    if let Err(_err) = which::which("git") {
        eprintln!("`git` is missing, please install it and retry");

        std::process::exit(1);
    }

    if let Err(_err) = which::which("cargo") {
        eprintln!(
            "`cargo` is missing, visit https://www.rust-lang.org/tools/install to install it"
        );

        std::process::exit(1);
    }
}

async fn check_crate_exists(crate_name: &str) -> eyre::Result<bool> {
    let client = reqwest::Client::new();
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);

    let response = client
        .get(&url)
        .header("User-Agent", "Crate Checker (your@email.com)")
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => Ok(true),
        StatusCode::NOT_FOUND => Ok(false),
        status => Err(eyre!("Unexpected status code: {}", status)),
    }
}

fn get_github_username() -> eyre::Result<String> {
    let output = Command::new("gh")
        .args(["api", "user", "--jq", ".login"])
        .output()?;

    if !output.status.success() {
        return Err(eyre!(
            "Failed to get GitHub username. Is 'gh' authenticated?"
        ));
    }

    String::from_utf8(output.stdout)
        .map(|s| s.trim().to_string())
        .map_err(|e| eyre!("Invalid UTF-8 in GitHub username: {}", e))
}

fn resolve_config_dir() -> eyre::Result<PathBuf> {
    let home = std::env::var("HOME")?;
    let config_dir = format!("{home}/.config/snatches");
    let config_dir = PathBuf::from(config_dir);

    std::fs::create_dir_all(&config_dir)?;

    Ok(config_dir)
}

fn resolve_config_file() -> eyre::Result<PathBuf> {
    let config_dir = resolve_config_dir()?;

    Ok(config_dir.join("config.toml"))
}

fn try_read_config(path: impl AsRef<Path>) -> eyre::Result<Option<Config>> {
    let path = path.as_ref();

    if !path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(path)?;

    if let Ok(config) = toml::from_str(&content) {
        Ok(Some(config))
    } else {
        Ok(None)
    }
}

fn try_initialize_config(path: impl AsRef<Path>) -> eyre::Result<Config> {
    println!("Looks like it's the first time running this utility");
    println!("we'll guide through a very short initialization");

    let path = path.as_ref();
    let gh_username = get_github_username()?;

    println!("The snatches repo is used to keep track of your snatched crates.");
    println!("It can also be used to request releasing a crate name");

    let default_repo_name = format!("{gh_username}/snatches");
    let prompt = inquire::Text::new("Snatches repo").with_default(&default_repo_name);
    let repo = prompt.prompt()?;

    let author = inquire::Text::new("Author - this will be used to populate the authors field in Cargo.toml. Feel free to skip it").prompt_skippable()?;

    let config = Config { repo, author };

    if let Some(parent_dir) = path.parent() {
        std::fs::create_dir_all(parent_dir)?;
    }

    let serialized = toml::to_string_pretty(&config)?;
    std::fs::write(path, serialized)?;

    Ok(config)
}

fn cargo_publish_repo(repo_dir: impl AsRef<Path>) -> eyre::Result<()> {
    let repo_dir = repo_dir.as_ref();
    let output = Command::new("cargo")
        .arg("publish")
        .current_dir(repo_dir)
        .output()?;

    if !output.status.success() {
        eprintln!("Failed to publish the crate");

        let stdout = String::from_utf8(output.stdout)?;
        let stderr = String::from_utf8(output.stderr)?;

        println!("{stdout}");
        eprintln!("{stderr}");

        std::process::exit(1);
    }

    Ok(())
}

fn check_github_repo_exists(repo: &str) -> eyre::Result<bool> {
    let output = Command::new("gh").args(["repo", "view", repo]).output()?;

    Ok(output.status.success())
}

fn create_snatches_repo(template_repo: &str, repo: &str) -> eyre::Result<()> {
    let output = Command::new("gh")
        .arg("repo")
        .arg("create")
        .arg(repo)
        .arg("--public")
        .arg("--template")
        .arg(template_repo)
        .output()?;

    if !output.status.success() {
        panic!("Failed to create the snatches repo!");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = Opt::parse();

    println!("args = {args:?}");

    ensure_deps();

    let config_file = resolve_config_file()?;
    let maybe_config = try_read_config(&config_file)?;

    let config = if let Some(config) = maybe_config {
        config
    } else {
        try_initialize_config(config_file)?
    };

    if !check_github_repo_exists(&config.repo)? {
        let repo = &config.repo;
        println!("{repo} doesn't exist - we'll create it in the next step");
        println!("The snatches repository is used to request the release of crate names");

        println!("Choose the template repo from which your snatches repo will be created - you can also create it yourself");
        let template = inquire::Text::new("Snatches template")
            .with_default("dzejkop/snatches-template")
            .prompt_skippable()?
            .expect("Missing snatches repo template");

        create_snatches_repo(&template, repo)?;
    }

    let name = args.name;
    if check_crate_exists(&name).await? {
        eprintln!("Crate `{}` already exists", name);

        std::process::exit(1);
    }

    let tmp_dir = tempfile::tempdir()?;

    let repo = format!("https://github.com/{}", config.repo);
    let template_context = Context {
        name: name.clone(),
        repo,
        repo_stub: config.repo.clone(),
        author: config.author.clone(),
    };

    let repo_dir = tmp_dir.path().join("repo");

    template::instance_in(&repo_dir, &template_context)?;

    cargo_publish_repo(repo_dir)?;

    Ok(())
}
