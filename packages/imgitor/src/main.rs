// #[macro_use]
// extern crate rocket;
extern crate imgitor;

use std::{path::Path, fs::File, io::Cursor};

use imgitor::{dotenv, read, write};

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
    // write(String::from("./lake/myfile.txt"), String::from("myfile.txt")).await;
    let obj = read(String::from("myfile.txt")).await;
    let url = obj.download_url(600).unwrap();
    println!("{:?}", url);
    let response = reqwest::get(url).await.unwrap();
    let path = Path::new("./lake/myfile.txt");
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let mut content = Cursor::new(response.bytes().await.unwrap());
    std::io::copy(&mut content, &mut file).unwrap();
}