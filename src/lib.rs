use std::{fs, process::Command};

#[derive(Debug)]
pub struct Addon {
    name: String,
    file_path: String,
    should_update: bool,
}

impl Addon {
    fn new(name: String, file_path: String) -> Self {
        Addon {
            name,
            file_path,
            should_update: false,
        }
    }
    fn update(&self) {
        Command::new("git")
            .arg("-C")
            .arg(&self.file_path)
            .arg("pull")
            .status()
            .expect("failed to execute process");
    }

    fn check_update(&mut self) -> bool {
        let last_local_commit = Command::new("git")
            .arg("-C")
            .arg(&self.file_path)
            .arg("rev-parse")
            .arg("HEAD")
            .output()
            .expect("failed to execute process");
        let last_distant_commit = Command::new("git")
            .arg("-C")
            .arg(&self.file_path)
            .arg("rev-parse")
            .arg("origin/HEAD")
            .output()
            .expect("failed to execute process");

        if last_local_commit.stdout == last_distant_commit.stdout {
            self.should_update = true;
        } else {
            self.should_update = false;
        }

        self.should_update
    }
}

pub fn prompt_update(mut addon_list_with_git: Vec<Addon>, prompt_for_update: bool) {
    if !prompt_for_update {
        println!("Some addons have updates available. Do you want to update them? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.to_lowercase().trim() == "y" {
            for addon in &mut addon_list_with_git {
                if addon.should_update {
                    println!("Updating {}...", addon.name);
                    addon.update();
                }
            }
            println!("All addons are up to date.");
        } else {
            println!("No addons were updated.");
        }
    } else {
        println!("All addons are up to date.");
    }
}

pub fn should_update_addon(addon_list_with_git: &mut Vec<Addon>, prompt_for_update: &mut bool) {
    for addon in addon_list_with_git {
        println!("- {} ", addon.name);
        let has_update = addon.check_update();
        if has_update {
            *prompt_for_update = true;
        }
    }
}

pub fn get_addon_list(
    folders: fs::ReadDir,
    addon_list_with_git: &mut Vec<Addon>,
) -> Result<(), std::io::Error> {
    for folder in folders {
        let dir = folder?;

        let addon_folder = fs::read_dir(&dir.path())?;

        if dir.path().is_dir() {
            for addon in addon_folder {
                let addon_file = addon?;
                match addon_file.path() {
                    path if path.is_dir() => {
                        let file_name = path.file_name().unwrap().to_str().unwrap();

                        if file_name == ".git" {
                            let dir_path = dir.path();
                            let addon_dir_name = dir_path.file_name().unwrap().to_str().unwrap();
                            let addon = Addon::new(
                                addon_dir_name.to_string(),
                                dir_path.to_str().unwrap().to_string(),
                            );
                            addon_list_with_git.push(addon);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    println!(
        "Found {} addons with .git folder:",
        addon_list_with_git.len()
    );

    Ok({})
}
