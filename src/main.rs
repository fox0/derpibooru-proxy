#![feature(decl_macro)]
#![warn(clippy::pedantic)]

mod api;
mod config;
mod models;

use crate::api::search_images;
use crate::models::SearchImages;

use dotenv::dotenv;
use rocket::response::Redirect;
use rocket::{get, routes, uri};
use rocket_contrib::templates::Template;
use serde::Serialize;

/// Главная страница
#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(images: page = _))
}

#[derive(Serialize)]
struct ContextImages {
    q: String,
    ls: SearchImages,
}

#[get("/images?<page>")]
fn images(page: Option<u32>) -> Result<Template, reqwest::Error> {
    let q = "*".to_string(); //todo get param
    let ls = search_images(&q, page)?;
    Ok(Template::render("images", &ContextImages { q, ls }))
}

#[get("/images/<id>")]
fn image(id: u32) -> Result<Template, reqwest::Error> {
    todo!()
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .mount("/", routes![index, images, image])
        .attach(Template::fairing())
        // todo custom error handler
        .launch();
}
