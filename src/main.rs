use std::fs;

use clap::Parser;
use raddon::{Addon, get_addon_list, prompt_update, should_update_addon};

/// A Rust CLI program that update your wow addon and install new one
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to your Addon Folder
    #[arg(short, long)]
    folder: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let folders = fs::read_dir(args.folder)?;

    let mut addon_list_with_git: Vec<Addon> = Vec::new();

    let mut prompt_for_update: bool = false;

    get_addon_list(folders, &mut addon_list_with_git)?;

    should_update_addon(&mut addon_list_with_git, &mut prompt_for_update);

    prompt_update(addon_list_with_git, prompt_for_update);

    Ok(())
}
