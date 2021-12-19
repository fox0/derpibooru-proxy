#![feature(decl_macro)]
#![warn(clippy::pedantic)]

mod api;
mod config;
mod models;
mod views;

use crate::views::{images, index};

use dotenv::dotenv;
use rocket::http::Method;
use rocket::Route;

fn main() {
    dotenv().ok();
    let routes = vec![
        Route::new(Method::Get, "/", index),
        Route::new(Method::Get, "/images", images),
        // Route::new(Method::Get, "/images?<page>", images),
    ];
    rocket::ignite().mount("/", routes).launch();
}
