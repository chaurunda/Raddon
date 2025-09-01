use std::{env, fs};

use clap::Parser;
use raddon::prompt_dir_path;

mod install;
mod update;

/// A Rust CLI program that update your wow addon and install new one
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    /// Optionnal : Path to your Addon Folder
    #[arg(short, long)]
    folder: Option<String>,

    /// Optionnal : Install addon from git url (folder must be specified or in path.txt)
    #[arg(short, long)]
    install: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let folder: String;
    let path = env::home_dir().unwrap().to_str().unwrap().to_owned() + "/.config/path.txt";
    println!("Config file path: {}", path);
    if args.folder.as_ref().map_or(true, |s| s.is_empty()) {
        folder =
            String::from_utf8(fs::read(&path).unwrap_or_else(|_| prompt_dir_path().into_bytes()))?;
    } else {
        folder = args.folder.unwrap();
    }
    fs::write(path, folder.as_bytes())?;
    let folders = fs::read_dir(&folder)?;

    if args.install.is_none() {
        update::update_addon(folders)?;
    }

    match args.install {
        Some(url) => {
            install::install(&url, &folder);
        }
        None => {}
    }

    Ok(())
}
