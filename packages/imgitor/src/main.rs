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
    // let file_name = String::from("myfile.txt");
    // let obj = read(&file_name).await;
    // let will_save_path = String::from("./lake/") + &file_name;
    // download(&obj.download_url(600).unwrap(), &will_save_path).await;
    // write(&will_save_path, &file_name).await;
    // let extracted_folder_path = extract("./lake/a.zip", &path::PathBuf::from("./lake"));
    // println!("{:?}", extracted_folder_path.to_str().unwrap());
    // let extracted_folder_path = String::from("./lake/a");
    // let dist_path = String::from("./lake/b.zip");
    // compress(&extracted_folder_path.as_str(), &dist_path.as_str());
}
