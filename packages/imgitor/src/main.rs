// #[macro_use]
// extern crate rocket;
extern crate imgitor;

use imgitor::{compress, dotenv, download, extract, read, write};
use std::path;

// #[get("/")]
// async fn index() -> &'static str {
//     dotenv().ok();
//     // read().await;
//     // write().await;
//     "Hello, world!"
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index])
// }

#[tokio::main]
async fn main() {
    dotenv().ok();
    let file_name = String::from("a.zip");
    let lake = String::from("./lake/");
    let obj = read(&file_name).await;
    let will_save_path = lake.clone() + &file_name;
    download(&obj.download_url(600).unwrap(), &will_save_path).await;
    let extracted_folder_path = extract(&will_save_path, &path::PathBuf::from(lake.clone()));
    let dist_path = will_save_path + &String::from(".custom.zip");
    compress(&extracted_folder_path.to_str().unwrap(), &dist_path);
    write(&dist_path, &file_name).await;
}
