use cloud_storage::{Client, Object};
use std::{env, fs::File, io::Cursor, io::Read, path::Path};

pub async fn read(n: &str) -> Object {
    println!("reading...");
    let bucket_name = env::var("GCP_CLOUD_STORAGE_READ_BUCKET_NAME")
        .expect("GCP_CLOUD_STORAGE_READ_BUCKET_NAME must be set");
    let object = Client::default()
        .object()
        .read(bucket_name.as_str(), n)
        .await
        .unwrap();
    object
}

pub async fn write(p: &str, n: &str) {
    println!("writing...");
    let bucket_name = env::var("GCP_CLOUD_STORAGE_WRITE_BUCKET_NAME")
        .expect("GCP_CLOUD_STORAGE_WRITE_BUCKET_NAME must be set");
    let mut bytes: Vec<u8> = Vec::new();
    for byte in File::open(p).unwrap().bytes() {
        bytes.push(byte.unwrap())
    }
    Client::default()
        .object()
        .create(bucket_name.as_str(), bytes, n, "text/plain")
        .await
        .unwrap();
    println!("writed");
}

pub async fn download(u: &str, p: &str) {
    let response = reqwest::get(u).await.unwrap();
    let path = Path::new(p);
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let mut content = Cursor::new(response.bytes().await.unwrap());
    std::io::copy(&mut content, &mut file).unwrap();
}
