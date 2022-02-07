use std::path;

extern crate rar;

pub fn extract(f: &str, p: &path::PathBuf) -> path::PathBuf {
    rar::Archive::extract_all(f, p.to_str().unwrap(), "").unwrap();
    return p.to_path_buf();
}
