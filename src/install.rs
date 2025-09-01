use raddon::{install_addon, rename_git_folder};

pub fn install(url: &str, folder: &str) {
    install_addon(url, folder);

    rename_git_folder(url, folder);
}
