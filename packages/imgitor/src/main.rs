// #[macro_use]
// extern crate rocket;
extern crate imgitor;

use imgitor::{dotenv, download, read, write};

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
    let file_name = String::from("myfile.txt");
    let obj = read(&file_name).await;
    let will_save_path = String::from("./lake/") + &file_name;
    download(&obj.download_url(600).unwrap(), &will_save_path).await;
    write(&will_save_path, &file_name).await;
}
