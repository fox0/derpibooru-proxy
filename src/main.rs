#![feature(decl_macro)]
#![warn(clippy::pedantic)]

mod api;
mod config;
mod models;

use crate::api::search_images;
use crate::config::CONFIG;
use crate::models::{Parameters, SearchImages};

use dotenv::dotenv;
use rocket::response::Redirect;
use rocket::{get, routes, uri};
use rocket_contrib::templates::Template;
use serde::Serialize;

/// Главная страница
#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(images: page = _, q = _, sf = _, sd = _))
}

#[derive(Serialize)]
struct ContextImages {
    params: Parameters,
    ls: SearchImages,
}

#[get("/images?<page>&<q>&<sd>&<sf>")]
fn images(
    page: Option<u32>,
    q: Option<String>,
    sf: Option<String>,
    sd: Option<String>,
) -> Result<Template, reqwest::Error> {
    let page = page.unwrap_or(1);
    let q = q.unwrap_or_else(|| "*".to_string());
    let params = Parameters {
        key: CONFIG.api_key.clone(), // абстракция протекла, и фиг с ней
        page,
        per_page: Some(40), // todo
        q,
        sf, // todo
        sd, // todo
    };
    let ls = search_images(&params)?;
    Ok(Template::render("images", &ContextImages { params, ls }))
}

// todo + tags
#[get("/images/<id>")]
fn image(id: u32) -> Result<Template, reqwest::Error> {
    todo!()
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .mount("/", routes![index, images, image])
        .attach(Template::fairing())
        .launch();
}
