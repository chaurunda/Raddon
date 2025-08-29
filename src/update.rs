use std::fs::ReadDir;

use raddon::{Addon, get_addon_list, prompt_update, should_update_addon};

pub fn update_addon(folders: ReadDir) -> anyhow::Result<()> {
    let mut addon_list_with_git: Vec<Addon> = Vec::new();

    let mut prompt_for_update: bool = false;

    let _ = get_addon_list(folders, &mut addon_list_with_git);

    should_update_addon(&mut addon_list_with_git, &mut prompt_for_update);

    prompt_update(addon_list_with_git, prompt_for_update);
    Ok(())
}
