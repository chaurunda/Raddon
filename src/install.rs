use std::path::Path;

use raddon::install_addon;
use wildpath::resolve;

pub fn install(url: &str, folder: &str) {
    // install_addon(url, folder);

    let splitted_url = url.split("/");

    let mut repo_with_git = String::new();

    for surl in splitted_url {
        if surl.contains(".git") {
            repo_with_git = surl.replace(".git", "");
        }
    }

    let repo_name = repo_with_git.as_str();

    println!("{}", &repo_name);
    println!("{}", &folder);
    println!("{}", &(folder.to_owned() + "/" + repo_name + "/*.toc"));

    let toc_file_wildcard = resolve(&Path::new(
        &(folder.to_owned() + "/" + repo_name + "/*.toc"),
    ));

    let mut toc_file_path = String::new();

    match toc_file_wildcard {
        Some(file) => toc_file_path = file[0].to_str().unwrap().to_owned(),
        None => {}
    }

    let split_toc_file_path = toc_file_path.split("/");
    let mut toc_file_name = "";

    for toc_name in split_toc_file_path {
        if toc_name.contains(".toc") {
            toc_file_name = &toc_name.replace(".toc", "")
        }
    }
}
