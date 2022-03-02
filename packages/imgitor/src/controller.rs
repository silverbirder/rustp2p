use crate::{compress, download, rar_extract, read, write, zip_extract, Transform, TransformTrait};
use std::path::PathBuf;

pub async fn index(n: &str) {
    let c = Controller::new(String::from(n));
    match c.file_type {
        FileType::UNKNOWN => {
            println!("Not support file");
            return;
        }
        _ => {
            c.process().await;
        }
    }
}

struct Controller {
    target_file_name: String,
    file_type: FileType,
}

#[derive(Debug)]
enum FileType {
    CBZ,
    ZIP,
    RAR,
    UNKNOWN,
}

impl Controller {
    fn new(target_file_name: String) -> Controller {
        let file_type = if target_file_name.ends_with(".rar") {
            FileType::RAR
        } else if target_file_name.ends_with(".zip") {
            FileType::ZIP
        } else if target_file_name.ends_with(".cbz") {
            FileType::CBZ
        } else {
            FileType::UNKNOWN
        };
        Controller {
            target_file_name: target_file_name,
            file_type: file_type,
        }
    }
    async fn process(self) {
        // download
        println!("downloading...");
        let obj = read(&self.target_file_name).await;
        let save_path = PathBuf::from(format!("./lake/{}", self.target_file_name));
        download(&obj.download_url(600).unwrap(), &save_path).await;
        println!("downloaded. save_path: {}", &save_path.to_str().unwrap());

        // extract
        println!("extracting...");
        let extract_path = save_path.with_extension(""); // remove extension
        match self.file_type {
            FileType::CBZ | FileType::ZIP => {
                zip_extract(&save_path, &extract_path);
            }
            FileType::RAR => {
                rar_extract(&save_path, &extract_path);
            }
            _ => {}
        }
        println!(
            "extracted. extract_path: {}",
            &extract_path.to_str().unwrap()
        );

        // transform
        println!("transforming...");
        let t = Transform {
            src_dir: &extract_path,
            thread_pool_num: 8,
        };
        println!("processing remove_not_image ...");
        t.walk_dir(Transform::remove_not_image, true);
        println!("processing convert ...");
        t.walk_dir(Transform::convert, true);
        println!("processing split ...");
        t.walk_dir(Transform::split, true);
        println!("processing resize ...");
        t.walk_dir(Transform::resize, true);
        println!("processing rename ...");
        t.rename();
        println!("transformed");

        // compress
        println!("compressing...");
        let compress_path = save_path.with_extension("custom.zip");
        compress(&extract_path, &compress_path);
        println!(
            "compressed. compress_path: {}",
            &compress_path.to_str().unwrap()
        );

        // upload
        println!("uploading...");
        write(
            &compress_path,
            compress_path.file_name().unwrap().to_str().unwrap(),
        )
        .await;
        println!("uploaded.");
    }
}
