use std::fs;

use clap::Parser;
use raddon::prompt_dir_path;

mod install;
mod update;

const PATH: &str = "./file.txt";

/// A Rust CLI program that update your wow addon and install new one
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    /// Path to your Addon Folder
    #[arg(short, long, default_value_t = String::from(""))]
    folder: String,

    /// install addon from git url
    #[arg(short, long)]
    install: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let folder: String;
    if args.folder.is_empty() {
        folder =
            String::from_utf8(fs::read(PATH).unwrap_or_else(|_| prompt_dir_path().into_bytes()))?;
    } else {
        folder = args.folder.to_string();
    }
    fs::write(PATH, folder.as_bytes())?;
    let folders = fs::read_dir(&folder)?;

    if args.install.is_none() {
        update::update_addon(folders)?;
    }

    match args.install {
        Some(url) => {
            install::install(&url, &args.folder);
        }
        None => {}
    }

    Ok(())
}
