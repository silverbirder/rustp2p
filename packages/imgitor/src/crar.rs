use std::path;
use std::process::Command;

pub fn extract(f: &str, p: &path::PathBuf) -> path::PathBuf {
    Command::new("7z")
        .arg("x")
        .arg("-o./lake/")
        .arg(f.to_string())
        .output()
        .expect("Failed to execute command");

    return p.to_path_buf();
}
