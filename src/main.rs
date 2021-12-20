#![feature(decl_macro)]
#![warn(clippy::pedantic)]

mod api;
mod config;
mod models;

use crate::api::{search_images, ApiError};
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
    // todo картинки + следилка + за последние 3 дня 4-8 картинок + feached
    Redirect::to(uri!(search: page = _, q = _, sf = _, sd = _))
}

#[get("/search?<page>&<q>&<sd>&<sf>")]
fn search(
    page: Option<u32>,
    q: Option<String>,
    sf: Option<String>,
    sd: Option<String>,
) -> Result<Template, ApiError> {
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
    let images = search_images(&params)?;
    {
        #[derive(Serialize)]
        struct Context {
            params: Parameters,
            images: SearchImages,
        }
        Ok(Template::render("images", &Context { params, images }))
    }
}

// todo + tags
#[get("/images/<id>")]
fn image(id: u32) -> Result<Template, reqwest::Error> {
    todo!()
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .mount("/", routes![index, search, image])
        .attach(Template::fairing())
        .launch();
}
