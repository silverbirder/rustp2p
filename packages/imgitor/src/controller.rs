use crate::{compress, download, rar_extract, read, write, zip_extract, Transform, TransformTrait};
use regex::Regex;
use std::path;

pub async fn index(n: &str) {
    let file_name = String::from(n);
    let lake = String::from("./lake/");
    let obj = read(&file_name).await;
    let will_save_path = lake.clone() + &file_name;
    download(&obj.download_url(600).unwrap(), &will_save_path).await;
    let extracted_folder_path;
    if file_name.ends_with(".rar") {
        let re = Regex::new(r"\.rar").unwrap();
        let file_name_exclude_suffix = re.replace_all(&file_name, "").to_string();
        extracted_folder_path = rar_extract(
            &will_save_path,
            &path::PathBuf::from(lake.clone() + &file_name_exclude_suffix),
        );
        let t = Transform {
            src_dir: &extracted_folder_path,
            thread_pool_num: 8,
        };
        t.walk_dir(Transform::convert, true);
        t.walk_dir(Transform::split, true);
        t.walk_dir(Transform::resize, true);
        t.rename();
        let dist_path = will_save_path + &String::from(".custom.zip");
        compress(&extracted_folder_path.to_str().unwrap(), &dist_path);
        write(
            &dist_path,
            &(file_name_exclude_suffix.clone() + &String::from(".zip")),
        )
        .await;
    } else if file_name.ends_with(".zip") || file_name.ends_with(".cbz") {
        zip_extract(&will_save_path, &path::PathBuf::from(lake.clone()));
        let re = Regex::new(r"\.(zip|cbz)").unwrap();
        let file_name_exclude_suffix = re.replace_all(&file_name, "").to_string();
        let extracted_folder_path = path::PathBuf::from(lake.clone() + &file_name_exclude_suffix);
        println!("{:?}", extracted_folder_path);
        let t = Transform {
            src_dir: &extracted_folder_path,
            thread_pool_num: 8,
        };
        t.walk_dir(Transform::convert, true);
        t.walk_dir(Transform::split, true);
        t.walk_dir(Transform::resize, true);
        t.rename();
        let dist_path = will_save_path + &String::from(".custom.zip");
        compress(&extracted_folder_path.to_str().unwrap(), &dist_path);
        write(&dist_path, &file_name).await;
    } else {
        return;
    }
}

struct Controller {}

#[derive(Debug)]
enum FILE_TYPE {
    CBZ,
    ZIP,
    RAR,
    UNKNOWN,
}

impl Controller {
    fn file_type(self)-> FILE_TYPE {
        FILE_TYPE::UNKNOWN
    }
    fn process(self, file_type: FILE_TYPE) {
        // download
        // extract
        // transform
        // compress
        // upload
        match file_type {
            FILE_TYPE::CBZ | FILE_TYPE::ZIP  => {},
            FILE_TYPE::RAR => {},
            FILE_TYPE::UNKNOWN => {},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controller_file_type() {
        let c = Controller{};
        let ft = c.file_type();
        match ft {
            FILE_TYPE::UNKNOWN => { assert!(true, "")},
            _ => {assert!(false, "No")},
        }
    }
}
