use std::{fs, io, path};

extern crate zip;

pub fn compress(f: &str) {
    println!("{:?}", f);
}
pub fn extract(f: &str, pb: &path::PathBuf) -> path::PathBuf {
    // TODO: Support file is only zip. need validation.
    let fname = std::path::Path::new(f);
    let file = fs::File::open(&fname).unwrap();
    let mut extracted_folder_path = path::PathBuf::from("");
    let mut archive = zip::ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let ii = format!("{:04}.png", i);
        let mut outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }
        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(pb.join(&outpath)).unwrap();
            let file_paths: Vec<&str> = file.name().split('/').collect();
            if file_paths.len() == 2 {
                extracted_folder_path = pb.join(file.name());
            }
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&pb.join(&p)).unwrap();
                }
            }
            // TODO: Check file. (skip if file suffix is not kind of image.)
            outpath.set_file_name(ii);
            let mut outfile = fs::File::create(pb.join(&outpath)).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&pb.join(&outpath), fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    return extracted_folder_path.clone();
}
