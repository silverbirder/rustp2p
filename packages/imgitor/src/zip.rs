use std::{
    fs::{self},
    io::{self, Read, Seek, Write},
    path,
};

use walkdir::{DirEntry, WalkDir};
use zip::{result::ZipError, write::FileOptions};

extern crate zip;

pub fn compress(src_dir: &path::PathBuf, dist_path: &path::PathBuf) {
    const METHOD_STORED: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Stored);
    #[cfg(any(
        feature = "deflate",
        feature = "deflate-miniz",
        feature = "deflate-zlib"
    ))]
    const METHOD_DEFLATED: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Deflated);
    #[cfg(not(any(
        feature = "deflate",
        feature = "deflate-miniz",
        feature = "deflate-zlib"
    )))]
    const METHOD_DEFLATED: Option<zip::CompressionMethod> = None;
    #[cfg(feature = "bzip2")]
    const METHOD_BZIP2: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Bzip2);
    #[cfg(not(feature = "bzip2"))]
    const METHOD_BZIP2: Option<zip::CompressionMethod> = None;
    #[cfg(feature = "zstd")]
    const METHOD_ZSTD: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Zstd);
    #[cfg(not(feature = "zstd"))]
    const METHOD_ZSTD: Option<zip::CompressionMethod> = None;
    for &method in [METHOD_STORED, METHOD_DEFLATED, METHOD_BZIP2, METHOD_ZSTD].iter() {
        if method.is_none() {
            continue;
        }
        match doit(src_dir, dist_path, method.unwrap()) {
            Ok(_) => println!("done: {} written to {}", src_dir.to_str().unwrap(), dist_path.to_str().unwrap()),
            Err(e) => println!("Error: {:?}", e),
        }
    }
    fn doit(
        src_dir: &path::PathBuf,
        dst_file: &path::PathBuf,
        method: zip::CompressionMethod,
    ) -> zip::result::ZipResult<()> {
        if !path::Path::new(src_dir).is_dir() {
            return Err(ZipError::FileNotFound);
        }
        let path = path::Path::new(dst_file);
        let file = fs::File::create(&path).unwrap();
        let walkdir = WalkDir::new(src_dir);
        let it = walkdir.into_iter();
        zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;
        Ok(())
    }

    fn zip_dir<T>(
        it: &mut dyn Iterator<Item = DirEntry>,
        prefix: &path::PathBuf,
        writer: T,
        method: zip::CompressionMethod,
    ) -> zip::result::ZipResult<()>
    where
        T: Write + Seek,
    {
        let mut zip = zip::ZipWriter::new(writer);
        let options = FileOptions::default()
            .compression_method(method)
            .unix_permissions(0o755);
        let mut buffer = Vec::new();
        for entry in it {
            let path = entry.path();
            let name = path.strip_prefix(path::Path::new(prefix)).unwrap();
            if path.is_file() {
                println!("adding file {:?} as {:?} ...", path, name);
                #[allow(deprecated)]
                zip.start_file_from_path(name, options).unwrap();
                let mut f = fs::File::open(path).unwrap();
                f.read_to_end(&mut buffer).unwrap();
                zip.write_all(&*buffer).unwrap();
                buffer.clear();
            } else if !name.as_os_str().is_empty() {
                println!("adding dir {:?} as {:?} ...", path, name);
                #[allow(deprecated)]
                zip.add_directory_from_path(name, options)?;
            }
        }
        zip.finish().unwrap();
        Result::Ok(())
    }
}

pub fn extract(f: &path::PathBuf, pb: &path::PathBuf) -> path::PathBuf {
    // TODO: Support file is only zip. need validation.
    // support zip (and cbz)
    let fname = std::path::Path::new(f);
    let file = fs::File::open(&fname).unwrap();
    let mut extracted_folder_path = path::PathBuf::from("");
    let mut archive = zip::ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        assert_eq!(1, 1);
    }
}
