use std::fs::{self};

mod git;
mod log;
use log::{ilog, log, slog, wlog};
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
        let _ = git::pull(&self.file_path);
    }

    fn check_update(&mut self) -> bool {
        let _ = git::fetch(&self.file_path);

        let last_local_commit = git::check_local_commit(&self.file_path);
        let last_distant_commit = git::check_distant_commit(&self.file_path);

        self.should_update = if last_local_commit != last_distant_commit {
            true
        } else {
            false
        };

        self.should_update
    }
}

pub fn prompt_update(mut addon_list_with_git: Vec<Addon>, prompt_for_update: bool) {
    if prompt_for_update {
        ilog("Some addons have updates available. Do you want to update them? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.to_lowercase().trim() == "y" {
            for addon in &mut addon_list_with_git {
                if addon.should_update {
                    log(&format!("Updating {}...", addon.name));
                    addon.update();
                }
            }
            log("All addons are up to date.");
        } else {
            log("No addons were updated.");
        }
    } else {
        log("All addons are up to date.");
    }
}

pub fn should_update_addon(addon_list_with_git: &mut Vec<Addon>, prompt_for_update: &mut bool) {
    for addon in addon_list_with_git {
        let has_update = addon.check_update();
        if has_update {
            wlog(&format!("- {} has update available", addon.name));
            addon.should_update = true;
            *prompt_for_update = true;
        } else {
            slog(&format!("- {} is up to date", addon.name));
        }
    }
}

pub fn get_addon_list(
    folders: fs::ReadDir,
    addon_list_with_git: &mut Vec<Addon>,
) -> anyhow::Result<()> {
    let dirs = folders
        .filter_map(|folder| folder.ok())
        .filter(|e| e.path().is_dir());
    for dir in dirs {
        let addon_folder = fs::read_dir(&dir.path())?;
        let addon_dirs = addon_folder
            .filter_map(|f| f.ok())
            .filter(|e| e.path().is_dir());
        for addon_dir in addon_dirs {
            match addon_dir.path() {
                path if path.is_dir() => {
                    let file_name = path.file_name().unwrap().to_string_lossy().to_string();

                    if file_name == ".git" {
                        let dir_path = dir.path();
                        let addon_dir_name = dir_path.file_name().unwrap().to_str().unwrap();
                        let addon = Addon::new(
                            addon_dir_name.to_string(),
                            dir_path.to_string_lossy().to_string(),
                        );
                        addon_list_with_git.push(addon);
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

pub fn install_addon(url: &str, folder: &str) {
    let _ = git::clone(url, folder);
}

#[cfg(test)]
mod tests {
    use std::{
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;

    fn unique_temp_dir() -> PathBuf {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let mut p = std::env::temp_dir();
        p.push(format!("raddon_test_{}", now));
        p
    }

    #[test]
    fn get_addon_list_finds_folders_with_dot_git() {
        // Arrange: create a unique temp directory structure:
        // temp/
        //   addon_with_git/
        //     .git/
        //   addon_without_git/
        let base = unique_temp_dir();
        let addon_with_git = base.join("addon_with_git");
        let addon_without_git = base.join("addon_without_git");

        fs::create_dir_all(addon_with_git.join(".git")).unwrap();
        fs::create_dir_all(&addon_without_git).unwrap();

        // Act
        let mut found: Vec<Addon> = Vec::new();
        let read_dir = fs::read_dir(&base).unwrap();
        let res = get_addon_list(read_dir, &mut found);

        // Cleanup
        let _ = fs::remove_dir_all(&base);

        // Assert
        assert!(res.is_ok());
        // Only the addon that contains a .git folder should be discovered
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name, "addon_with_git");
        // file_path should point to the addon directory
        assert!(found[0].file_path.ends_with("addon_with_git"));
    }
}
