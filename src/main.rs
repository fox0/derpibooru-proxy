#![feature(decl_macro)]
#![warn(clippy::pedantic)]

mod api;
mod config;
mod models;

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
) -> Result<Template, api::Error> {
    let params = Parameters::new(page, q, sf, sd);
    let images = api::search_images(&params)?;
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
#[get("/images/<image_id>")]
fn image(image_id: u32) -> Result<String, api::Error> {
    let image = api::images(image_id)?;
    Ok("ok".into())
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .mount("/", routes![index, search, image])
        .attach(Template::fairing())
        .launch();
}
