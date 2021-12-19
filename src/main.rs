#![feature(decl_macro)]
#![warn(clippy::pedantic)]

mod api;
mod config;

use crate::api::search_images;

use dotenv::dotenv;
use rocket::{get, routes};

#[get("/")]
fn index() -> Result<String, reqwest::Error> {
    search_images("*")
}

// #[get("/<name>/<age>")]
// fn hello(name: String, age: u8) -> String {
//     format!("Hello, {} year old named {}!", age, name)
// }

fn main() {
    dotenv().ok();
    rocket::ignite()
        .mount("/", routes![index])
        // .mount("/hello", routes![hello])
        // todo custom error handler
        .launch();
}
