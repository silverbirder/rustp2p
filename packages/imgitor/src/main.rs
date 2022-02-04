#[macro_use]
extern crate rocket;
extern crate imgitor;

use imgitor::{compress, dotenv, download, extract, read, rename, write};
use std::path;

// #[get("/?<n>")]
// async fn index(n: &str) -> &'static str {
//     process(n).await;
//     "Finished!"
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index])
// }

async fn process(n: &str) {
    dotenv().ok();
    rename();
    // let file_name = String::from(n);
    // let lake = String::from("./lake/");
    // let obj = read(&file_name).await;
    // let will_save_path = lake.clone() + &file_name;
    // download(&obj.download_url(600).unwrap(), &will_save_path).await;
    // let extracted_folder_path = extract(&will_save_path, &path::PathBuf::from(lake.clone()));
    // let dist_path = will_save_path + &String::from(".custom.zip");
    // compress(&extracted_folder_path.to_str().unwrap(), &dist_path);
    // write(&dist_path, &file_name).await;
}

fn main() {
    rename();
}