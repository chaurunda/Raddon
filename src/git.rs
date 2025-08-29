use anyhow::Result;
use std::process::Command;
pub fn fetch(file_path: &str) -> Result<()> {
    let result = Command::new("git")
        .arg("-C")
        .arg(file_path)
        .arg("fetch")
        .status();

    result.map(|_| ()).map_err(|e| e.into())
}

pub fn check_local_commit(file_path: &str) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(file_path)
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn check_distant_commit(file_path: &str) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(file_path)
        .arg("rev-parse")
        .arg("origin/HEAD")
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn pull(file_path: &str) -> Result<()> {
    println!("Pulling updates from remote repository...");
    let result = Command::new("git")
        .arg("-C")
        .arg(file_path)
        .arg("pull")
        .status();
    result.map(|_| ()).map_err(|e| e.into())
}

pub fn clone(repo_url: &str, destination: &str) -> Result<()> {
    println!("Cloning repository from {} to {}", repo_url, &destination);

    let path = std::path::Path::new(destination);
    let result = Command::new("git")
        .arg("clone")
        .arg(repo_url)
        .current_dir(path)
        .status();

    result.map(|_| ()).map_err(|e| e.into())
}
