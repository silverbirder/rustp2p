use walkdir::WalkDir;
use image::{GenericImageView};
use image::io::Reader as ImageReader;

pub fn rename() {
    // let wark = WalkDir::new("./lake/a").sort_by_file_name();
    // for a in wark {
    //     let dir = a.unwrap();
    //     println!("{:?}", dir.file_name());
    //     println!("{:?}", dir.file_type().is_dir());
    //     println!("{:?}", dir.file_type().is_file());
    //     println!("{:?}", dir.path());
    // }
    let img = ImageReader::open("./lake/a/0001.png").unwrap().decode().unwrap();
    // let img = image::open("./lake/a/0001.png").unwrap();
    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());
    img.save("./lake/a/0001.jpeg").unwrap();
}

pub fn reimage() {
    println!("reimage");
}

pub fn reduce() {
    println!("reduce");
}