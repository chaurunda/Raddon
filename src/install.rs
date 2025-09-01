use std::path::Path;

use raddon::install_addon;
use wildpath::resolve;

pub fn install(url: &str, folder: &str) {
    // install_addon(url, folder);

    let splitted_url = url.split("/");

    let mut repo_with_git = "";

    for surl in splitted_url {
        if surl.contains(".git") {
            repo_with_git = surl;
        }
    }

    let mut repo_name = "";

    for name in repo_with_git.split('.') {
        if name != "git" {
            repo_name = name;
        }
    }
    println!("{}", &repo_name);
    println!("{}", &folder);
    println!("{}", &(folder.to_owned() + "/" + repo_name + "/*.toc"));

    let toc_file_wildcard = resolve(&Path::new(
        &(folder.to_owned() + "/" + repo_name + "/*.toc"),
    ));

    let mut toc_file_path = "";

    match toc_file_wildcard {
        Some(file) => toc_file_path = file[0].to_str().unwrap(),
        None => {}
    }

    let split_toc_file_path = toc_file_path.split("/");
    let mut toc_file_name = "";

    for toc_name in split_toc_file_path {
        if toc_name.contains(".toc") {
            toc_file_name = toc_name
        }
    }
}
