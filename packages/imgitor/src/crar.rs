use std::path;

extern crate unrar;

pub fn extract(f: &str, p: &path::PathBuf) -> path::PathBuf {
    unrar::Archive::new(f.to_string())
        .extract_to(p.to_str().unwrap().to_string())
        .unwrap()
        .process()
        .unwrap();
    return p.to_path_buf();
}
