#[macro_use]
extern crate rocket;
extern crate imgitor;

use imgitor::{dotenv, index_routing};

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![index_routing])
}
