use image::GenericImageView;
use walkdir::WalkDir;
use regex::Regex;

pub fn rename() {
    let wark = WalkDir::new("./lake/a").sort_by_file_name();
    let re = Regex::new(r"/[^.]+\.(jpg|png)$").unwrap();
    let mut i = 0;
    for a in wark {
        let dir = a.unwrap();
        if dir.file_type().is_dir() {
            continue;
        }
        let result = image::open(dir.path());
        if result.is_err() {
            continue;
        } 
        let mut img = result.unwrap();
        let s = format!("{:08}", i);
        let name = String::from("/") + s.as_str() + &String::from(".jpeg");
        let path = re.replace_all(dir.path().to_str().unwrap(), name).to_string();
        println!("{:?}", path);
        if img.width() > img.height() {
            println!(">");
        };
        // while img.width() * img.height() > 500 * 1000 {
        //     println!("{:?}", img.dimensions());
        //     let w = img.width() as f64;
        //     let h = img.height() as f64;
        //     img = img.resize(
        //         ((w * 0.9).round() as i64).try_into().unwrap(),
        //         ((h * 0.9).round() as i64).try_into().unwrap(),
        //         image::imageops::FilterType::CatmullRom,
        //     );
        // }
        i = i + 1;
        // img.save(path).unwrap();
    }
    // img.crop(0, 0, 764, 600)
}

pub fn reimage() {
    println!("reimage");
}

pub fn reduce() {
    println!("reduce");
}
