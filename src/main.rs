use std::fs;

use clap::Parser;

mod install;
mod update;

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
    let folders = fs::read_dir(&args.folder)?;

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
