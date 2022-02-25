use std::{io::Write, ops::Deref, path::PathBuf, sync::mpsc};

use image::{DynamicImage, EncodableLayout};
use std::fs::File;
use threadpool::ThreadPool;
use walkdir::WalkDir;
use webp::Encoder;
pub struct Transform<'a> {
    pub src_dir: &'a PathBuf,
    pub thread_pool_num: usize,
}

pub trait TransformTrait {
    fn walk_dir<F: (Fn(&PathBuf, bool) -> PathBuf) + Send + Clone + 'static>(
        &self,
        f: F,
        update: bool,
    ) -> usize;
    fn rename(&self);
    fn encode_webp(img: DynamicImage, path: &PathBuf);
    fn convert(p: &PathBuf, update: bool) -> PathBuf;
    fn resize(p: &PathBuf, update: bool) -> PathBuf;
    fn split(p: &PathBuf, update: bool) -> PathBuf;
}

impl TransformTrait for Transform<'_> {
    fn walk_dir<F: (Fn(&PathBuf, bool) -> PathBuf) + Send + Clone + 'static>(
        &self,
        f: F,
        update: bool,
    ) -> usize {
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
                            let p = moved_f(&PathBuf::from(dir.path()), update);
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
    fn rename(&self) {
        let wark = WalkDir::new(&self.src_dir).sort_by_file_name();
        let mut inc = 1;
        for file in wark {
            let dir = file.unwrap();
            if dir.file_type().is_file() {
                let p = dir.into_path();
                let e = p.extension().unwrap();
                let file_name = format!("{:03}.{}", inc, e.to_str().unwrap());
                let rename_path = p.with_file_name(file_name);
                std::fs::rename(p.as_path(), rename_path).unwrap();
                inc = inc + 1;
            }
        }
    }
    fn encode_webp(img: DynamicImage, path: &PathBuf) {
        let widht = img.width();
        let height = img.height();
        let rgb_img = img.into_rgb8();
        let wpm = Encoder::from_rgb(rgb_img.as_bytes(), widht, height).encode(100.0);
        let wpmd = wpm.deref();
        let mut file = File::create(&path).unwrap();
        file.write_all(wpmd).unwrap();
        file.flush().unwrap();
    }
    fn convert(p: &PathBuf, update: bool) -> PathBuf {
        let img = image::open(p.as_path()).unwrap();
        let webp_path = PathBuf::from(format!("{}.webp", p.as_path().display()));
        Self::encode_webp(img, &webp_path);
        if update {
            std::fs::remove_file(p).unwrap();
        }
        return webp_path;
    }
    fn resize(p: &PathBuf, update: bool) -> PathBuf {
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
        let webp_path = PathBuf::from(format!("{}.resize.webp", p.as_path().display()));
        Self::encode_webp(img, &webp_path);
        if update {
            std::fs::remove_file(p).unwrap();
        }
        return PathBuf::from(webp_path);
    }
    fn split(p: &PathBuf, update: bool) -> PathBuf {
        let mut img = image::open(p.as_path()).unwrap();
        if img.height() >= img.width() {
            let a = p.to_path_buf();
            return a;
        }
        let left_img = img.crop(0, 0, img.width() / 2, img.height());
        let left_webp_path = PathBuf::from(format!("{}.split.left.webp", p.as_path().display()));
        Self::encode_webp(left_img, &left_webp_path);

        let right_img = img.crop(img.width() / 2, 0, img.width() / 2, img.height());
        let right_webp_path = PathBuf::from(format!("{}.split.right.webp", p.as_path().display()));
        Self::encode_webp(right_img, &right_webp_path);

        if update {
            std::fs::remove_file(p).unwrap();
        }
        return left_webp_path;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn transform_pass_walk_dir() {
        // Arrange
        let t = Transform {
            src_dir: &PathBuf::from("./samples/transform_pass_walk_dir"),
            thread_pool_num: 8,
        };
        fn sample(p: &PathBuf, _: bool) -> PathBuf {
            p.to_path_buf()
        }

        // Act
        let result = t.walk_dir(sample, false);

        // Assert
        assert_eq!(result, 1);
    }

    #[test]
    fn transform_convert() {
        // Arrange
        let p = PathBuf::from("./samples/transform_convert/rust-social-wide.jpeg");

        // Act
        let result = Transform::convert(&p, false);

        // Assert
        assert_eq!(
            result.to_str().unwrap(),
            "./samples/transform_convert/rust-social-wide.jpeg.webp"
        );

        // Teardown
        std::fs::remove_file("./samples/transform_convert/rust-social-wide.jpeg.webp").unwrap();
    }

    #[test]
    fn transform_resize() {
        // Arrange
        let p = PathBuf::from("./samples/transform_resize/rust-social-wide.webp");

        // Act
        let result = Transform::resize(&p, false);

        // Assert
        assert_eq!(
            result.to_str().unwrap(),
            "./samples/transform_resize/rust-social-wide.webp.resize.webp"
        );

        // Teardown
        std::fs::remove_file("./samples/transform_resize/rust-social-wide.webp.resize.webp")
            .unwrap();
    }

    #[test]
    fn transform_split() {
        // Arrange
        let p = PathBuf::from("./samples/transform_split/rust-social-wide.webp");

        // Act
        let result = Transform::split(&p, false);

        // Assert
        assert_eq!(
            result.to_str().unwrap(),
            "./samples/transform_split/rust-social-wide.webp.split.left.webp"
        );

        // Teardown
        std::fs::remove_file("./samples/transform_split/rust-social-wide.webp.split.left.webp")
            .unwrap();
        std::fs::remove_file("./samples/transform_split/rust-social-wide.webp.split.right.webp")
            .unwrap();
    }

    #[test]
    fn transform_encode_webp() {
        // Arrange
        let p = PathBuf::from("./samples/transform_encode_webp/rust-social-wide.jpeg");
        let img = image::open(p).unwrap();
        let path = PathBuf::from("./samples/transform_encode_webp/rust-social-wide.webp");

        // Act
        Transform::encode_webp(img, &path);

        // Assert
        assert!(path.is_file(), "save file");

        // Teardown
        std::fs::remove_file("./samples/transform_encode_webp/rust-social-wide.webp").unwrap();
    }

    #[test]
    fn transform_rename() {
        // Arrange
        let t = Transform {
            src_dir: &PathBuf::from("./samples/transform_rename"),
            thread_pool_num: 8,
        };

        // Act
        t.rename();

        // Assert
    }
}
