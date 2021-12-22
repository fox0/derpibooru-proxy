#![feature(decl_macro)]
#![warn(clippy::pedantic)]

mod api;
mod config;
mod errors;
mod models;
mod templates;

use crate::errors::Error;
use crate::models::Parameters;
use crate::templates::TEMPLATES;

use dotenv::dotenv;
use rocket::response::content::Html;
use rocket::response::Redirect;
use rocket::{get, routes, uri, Rocket};
use tera::Context;

/// Главная страница
#[get("/")]
fn index() -> Redirect {
    // todo картинки + следилка + за последние 3 дня 4-8 картинок + feached
    Redirect::to(uri!(search: page = _, q = _, sf = _, sd = _))
}

#[allow(clippy::needless_pass_by_value)]
#[get("/search?<page>&<q>&<sd>&<sf>")]
fn search(
    page: Option<u32>,
    q: Option<String>,
    sf: Option<String>,
    sd: Option<String>,
) -> Result<Html<String>, Error> {
    let params = Parameters::new(page, q, sf, sd);
    let images = api::search_images(&params)?;

    let mut context = Context::new();
    context.insert("params", &params);
    context.insert("images", &images);
    Ok(Html(TEMPLATES.render("search", &context)?))
}

// todo + tags
#[get("/images/<image_id>")]
fn image(image_id: u32) -> Result<String, Error> {
    let image = api::images(image_id)?;
    Ok("ok".into())
}

fn main() {
    dotenv().ok();
    Rocket::ignite()
        .mount("/", routes![index, search, image])
        .launch();
}
