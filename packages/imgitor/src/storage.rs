use cloud_storage::{Client, Object};
use std::{env, fs::File, io::Read};

pub async fn read(f: String) -> Object{
    println!("reading...");
    let bucket_name = env::var("GCP_CLOUD_STORAGE_BUCKET_NAME")
        .expect("GCP_CLOUD_STORAGE_BUCKET_NAME must be set");
    let object = Client::default()
        .object()
        .read(bucket_name.as_str(), &f)
        .await
        .unwrap();
    object
}

pub async fn write(f: String, n: String) {
    println!("writing...");
    let bucket_name = env::var("GCP_CLOUD_STORAGE_BUCKET_NAME")
        .expect("GCP_CLOUD_STORAGE_BUCKET_NAME must be set");
    let mut bytes: Vec<u8> = Vec::new();
    for byte in File::open(f).unwrap().bytes() {
        bytes.push(byte.unwrap())
    }
    Client::default()
        .object()
        .create(bucket_name.as_str(), bytes, &n, "text/plain")
        .await
        .unwrap();
    println!("writed");
}
