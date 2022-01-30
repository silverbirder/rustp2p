use cloud_storage::Client;
use std::{env, fs::File, io::Read};

pub async fn read() {
    println!("reading...");
    let bucket_name = env::var("GCP_CLOUD_STORAGE_BUCKET_NAME")
        .expect("GCP_CLOUD_STORAGE_BUCKET_NAME must be set");
    let mut object = Client::default()
        .object()
        .read(bucket_name.as_str(), "myfile.txt")
        .await
        .unwrap();
    println!("{:?}", object);
}

pub async fn write() {
    println!("writing...");
    let bucket_name = env::var("GCP_CLOUD_STORAGE_BUCKET_NAME")
        .expect("GCP_CLOUD_STORAGE_BUCKET_NAME must be set");
    let mut bytes: Vec<u8> = Vec::new();
    for byte in File::open("myfile.txt").unwrap().bytes() {
        bytes.push(byte.unwrap())
    }
    Client::default()
        .object()
        .create(bucket_name.as_str(), bytes, "myfile.txt", "text/plain")
        .await
        .unwrap();
    println!("writed");
}
