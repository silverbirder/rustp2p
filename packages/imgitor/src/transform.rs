use std::{io::Write, ops::Deref, path::PathBuf, sync::mpsc};

use image::GenericImageView;
use std::fs::File;
use threadpool::ThreadPool;
use walkdir::WalkDir;
use webp::Encoder;

pub fn rename(p: &str) {
    let wark = WalkDir::new(p).sort_by_file_name();
    let mut i = -1;
    let pool = ThreadPool::new(8);
    for a in wark {
        pool.execute(move || {
            let dir = a.unwrap();
            if dir.file_type().is_dir() {
                return;
            }
            let result = image::open(dir.path());
            if result.is_err() {
                return;
            }
            let mut img = result.unwrap();
            while img.width() * img.height() > 500 * 1000 {
                println!("{:?}", img.dimensions());
                let w = img.width() as f64;
                let h = img.height() as f64;
                img = img.resize(
                    ((w * 0.5).round() as i64).try_into().unwrap(),
                    ((h * 0.5).round() as i64).try_into().unwrap(),
                    image::imageops::FilterType::CatmullRom,
                );
            }
            let s = format!("{:08}", i);
            let dir_path = dir
                .path()
                .to_str()
                .unwrap()
                .replace(dir.file_name().to_str().unwrap(), "");
            let dist_path = dir_path + &s + ".jpg";
            println!("{:?}", dist_path);
            println!("{:?}", dir.path());
            img.save(dist_path).unwrap();
            std::fs::remove_file(dir.path()).unwrap();
        });
        i = i + 1;
    }
    pool.join();
    // for a in wark {
    //     let dir = a.unwrap();
    //     if dir.file_type().is_dir() {
    //         continue;
    //     }
    //     let result = image::open(dir.path());
    //     if result.is_err() {
    //         continue;
    //     }
    //     let mut img = result.unwrap();
    //     let s = format!("{:08}", i);
    //     let dir_path = dir.path().to_str().unwrap().replace(dir.file_name().to_str().unwrap(), "");
    //     let dist_path = dir_path + &s + ".jpeg";
    //     if img.width() > img.height() {
    //         let mut left_img = img.crop(0, 0, img.width() / 2, img.height());
    //         let mut right_img = img.crop(img.width() / 2, 0, img.width() / 2, img.height());
    //         while left_img.width() * left_img.height() > 500 * 1000 {
    //             println!("{:?}", left_img.dimensions());
    //             let w = left_img.width() as f64;
    //             let h = left_img.height() as f64;
    //             left_img = left_img.resize(
    //                 ((w * 0.9).round() as i64).try_into().unwrap(),
    //                 ((h * 0.9).round() as i64).try_into().unwrap(),
    //                 image::imageops::FilterType::CatmullRom,
    //             );
    //         }
    //         left_img.save(dist_path).unwrap();
    //         i = i + 1;

    //         while right_img.width() * right_img.height() > 500 * 1000 {
    //             println!("{:?}", right_img.dimensions());
    //             let w = right_img.width() as f64;
    //             let h = right_img.height() as f64;
    //             right_img = right_img.resize(
    //                 ((w * 0.9).round() as i64).try_into().unwrap(),
    //                 ((h * 0.9).round() as i64).try_into().unwrap(),
    //                 image::imageops::FilterType::CatmullRom,
    //             );
    //         }
    //         let s = format!("{:08}", i);
    //         let dir_path = dir.path().to_str().unwrap().replace(dir.file_name().to_str().unwrap(), "");
    //         let dist_path = dir_path + &s + ".jpeg";
    //         right_img.save(dist_path).unwrap();
    //         i = i + 1;
    //     } else {
    //         while img.width() * img.height() > 500 * 1000 {
    //             println!("{:?}", img.dimensions());
    //             let w = img.width() as f64;
    //             let h = img.height() as f64;
    //             img = img.resize(
    //                 ((w * 0.9).round() as i64).try_into().unwrap(),
    //                 ((h * 0.9).round() as i64).try_into().unwrap(),
    //                 image::imageops::FilterType::CatmullRom,
    //             );
    //         }
    //         i = i + 1;
    //         img.save(dist_path).unwrap();
    //     }
    // }
}

struct Transform {
    src_dir: PathBuf,
    thread_pool_num: usize,
}

pub trait TransformTrait {
    fn check_fields(&self) -> Result<i64, String>;
    fn walk_dir<F: (Fn(&PathBuf) -> PathBuf) + Send + Clone + 'static>(&self, f: F) -> usize;
    fn convert(p: &PathBuf) -> PathBuf;
    fn resize(p: &PathBuf) -> PathBuf;
}

impl TransformTrait for Transform {
    fn check_fields(&self) -> Result<i64, String> {
        if !self.src_dir.is_dir() {
            return Err(format!(
                "{:?} is not directory. expected value is directory.",
                self.src_dir
            ));
        }
        Ok(1)
    }
    fn walk_dir<F: (Fn(&PathBuf) -> PathBuf) + Send + Clone + 'static>(&self, f: F) -> usize {
        let wark = WalkDir::new(&self.src_dir).sort_by_file_name();
        let pool = ThreadPool::new(self.thread_pool_num);
        let (tx, rx) = mpsc::channel();
        let mut inc = 0;
        for file in wark {
            match file {
                Err(_) => {}
                Ok(dir) => {
                    if dir.file_type().is_file() {
                        let moved_f = f.to_owned();
                        let moved_tx = tx.to_owned();
                        pool.execute(move || {
                            let p = moved_f(&PathBuf::from(dir.path()));
                            moved_tx.send(p).unwrap();
                        });
                        inc = inc + 1;
                    }
                }
            }
        }
        pool.join();
        let count = rx.iter().take(inc).count();
        return count;
    }
    fn convert(p: &PathBuf) -> PathBuf {
        let img = image::open(p.as_path()).unwrap();
        let wpm = Encoder::from_image(&img).unwrap().encode_lossless();
        let wpmd = wpm.deref();
        let webp_path = format!("{}.webp", p.as_path().display());
        let mut file = File::create(&webp_path).unwrap();
        file.write_all(wpmd).unwrap();
        file.flush().unwrap();
        return PathBuf::from(webp_path);
    }
    fn resize(p: &PathBuf) -> PathBuf {
        let mut img = image::open(p.as_path()).unwrap();
        while img.width() * img.height() > 500 * 1000 {
            let w = img.width() as f64;
            let h = img.height() as f64;
            img = img.resize(
                ((w * 0.9).round() as i64).try_into().unwrap(),
                ((h * 0.9).round() as i64).try_into().unwrap(),
                image::imageops::FilterType::CatmullRom,
            );
        }
        let wpm = Encoder::from_image(&img).unwrap().encode_lossless();
        let wpmd = wpm.deref();
        let webp_path = format!("{}.resize.webp", p.as_path().display()); // todo: delete suffix webp.
        let mut file = File::create(&webp_path).unwrap();
        file.write_all(wpmd).unwrap();
        file.flush().unwrap();
        return PathBuf::from(webp_path);
    }
}

#[cfg(test)]
mod tests {
    use crate::transform::Transform;
    use std::path::PathBuf;

    use super::TransformTrait;

    #[test]
    fn transform_check_fields_is_ok() {
        // Arrange
        let t = Transform {
            src_dir: PathBuf::from("."),
            thread_pool_num: 8,
        };

        // Act
        let result = t.check_fields();

        // Assert
        match result {
            Ok(_) => assert!(true, ""),
            Err(m) => assert!(false, "{:?}", m),
        }
    }

    #[test]
    fn transform_check_fields_is_ng() {
        // Arrange
        let t = Transform {
            src_dir: PathBuf::from("./README.md"),
            thread_pool_num: 8,
        };

        // Act
        let result = t.check_fields();

        // Assert
        match result {
            Ok(_) => assert!(false, ""),
            Err(_) => assert!(true, ""),
        }
    }

    #[test]
    fn transform_pass_walk_dir() {
        // Arrange
        let t = Transform {
            src_dir: PathBuf::from("./samples/transform_pass_walk_dir"),
            thread_pool_num: 8,
        };
        fn sample(p: &PathBuf) -> PathBuf {
            p.to_path_buf()
        }

        // Act
        let result = t.walk_dir(sample);

        // Assert
        assert_eq!(result, 1);
    }

    #[test]
    fn transform_convert() {
        // Arrange
        let p = PathBuf::from("./samples/transform_convert/rust-social-wide.jpeg");

        // Act
        let result = Transform::convert(&p);

        // Assert
        assert_eq!(
            result.to_str().unwrap(),
            "./samples/transform_convert/rust-social-wide.jpeg.webp"
        );

        // Teardown
        std::fs::remove_file("./samples/transform_convert/rust-social-wide.jpeg.webp").unwrap();
    }

    #[test]
    fn transform_rename() {}

    #[test]
    fn transform_resize() {
        // Arrange
        let p = PathBuf::from("./samples/transform_resize/rust-social-wide.webp");

        // Act
        let result = Transform::resize(&p);

        // Assert
        assert_eq!(
            result.to_str().unwrap(),
            "./samples/transform_resize/rust-social-wide.webp.resize.webp"
        );

        // Teardown
        std::fs::remove_file("./samples/transform_resize/rust-social-wide.webp.resize.webp").unwrap();
    }

    #[test]
    fn transform_split() {}
}
