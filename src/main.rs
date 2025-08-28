use std::{fs, path, process::Command};

use clap::Parser;

/// A Rust CLI program that update your wow addon and install new one 
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to your Addon Folder
    #[arg(short, long)]
    folder: String,
}

#[derive(Debug)]
struct Addon {
    name: String,
    file_path: String,
    should_update: bool,
}

impl Addon {
    fn new(name: String, file_path: String) -> Self {
        Addon { name, file_path, should_update: false }
    }
    fn update(&self) {
        Command::new("git")
            .arg("-C")
            .arg(&self.file_path)
            .arg("pull")
            .status()
            .expect("failed to execute process");
    }

    fn check_update(&self) -> bool {
        let output = Command::new("git")
            .arg("-C")
            .arg(&self.file_path)
            .arg("fetch")
            .arg("--dry-run")
            .output()
            .expect("failed to execute process");

        !output.stdout.is_empty()
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let folders = fs::read_dir(args.folder)?;

    let mut addon_list_with_git: Vec<Addon> = Vec::new();

    for folder in folders {
        let dir = folder?;

        let addon_folder = fs::read_dir(&dir.path())?;

        for addon in addon_folder {
            let addon_file = addon?;
            match addon_file.path() {
                path     if path.is_dir() => {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    
                    if file_name == ".git" {
                        let dir_path = dir.path();
                        let addon_dir_name = dir_path.file_name().unwrap().to_str().unwrap();
                        let addon = Addon::new(addon_dir_name.to_string(), dir_path.to_str().unwrap().to_string());
                        addon_list_with_git.push(addon);
                    }
                }
                _ => {}
            } 
        }
    }

    println!("Found {} addons with .git folder:", addon_list_with_git.len());
    for addon in addon_list_with_git {
        println!("- {} ", addon.name);
        addon.update();
    }
    Ok(())
}
// /home/chaurunda/Games/turtlewow/drive_c/'Program Files (x86)'/TurtleWoW/Interface/AddOns