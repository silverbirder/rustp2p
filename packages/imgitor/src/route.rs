use crate::index_controller;
use rocket::get;

#[get("/?<n>")]
pub async fn index(n: &str) -> &'static str {
    index_controller(n).await;
    "Finished!"
}
