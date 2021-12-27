#![feature(decl_macro)]
#![warn(clippy::pedantic)]

mod api;
mod config;
mod errors;
mod models;
mod templates;

use crate::errors::Error;
use crate::models::{Pagination, Parameters};
use crate::templates::{render, Template};

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
) -> Result<Template, Error> {
    let params = Parameters::new(page, q, sf, sd);
    let images = api::search_images(&params)?;
    let pagination = Pagination::new(params.page.unwrap(), images.total);

    let mut context = Context::new();
    context.insert("params", &params);
    context.insert("images", &images);
    context.insert("pagination", &pagination);
    render("search", &mut context)
}

#[get("/images/<_image_id>")]
fn image(_image_id: u32) -> Result<Template, Error> {
    let mut context = Context::new();
    render("image", &mut context)
}

fn main() {
    use dotenv::dotenv;
    dotenv().ok();
    // lazy_static::initialize(&CONFIG);
    Rocket::ignite()
        .mount("/", routes![index, search, image])
        .launch();
}
