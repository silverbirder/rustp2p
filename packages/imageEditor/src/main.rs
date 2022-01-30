extern crate imgitor;

use dotenv::dotenv;
use imgitor::{read, write};

#[tokio::main]
async fn main() {
    dotenv().ok();
    read().await;
    // write().await;
}
