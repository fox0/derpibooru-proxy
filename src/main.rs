#![feature(decl_macro)]
#![warn(clippy::pedantic)]

mod api;
mod config;
mod errors;
mod models;

use crate::errors::Error;
use crate::models::{Parameters, SearchImages};

use dotenv::dotenv;
use rocket::response::{content, Redirect};
use rocket::{get, routes, uri, Rocket, State};
use serde::Serialize;
use tera::{Context as TeraContext, Tera};

/// Главная страница
#[get("/")]
fn index() -> Redirect {
    // todo картинки + следилка + за последние 3 дня 4-8 картинок + feached
    Redirect::to(uri!(search: page = _, q = _, sf = _, sd = _))
}

#[allow(clippy::needless_pass_by_value)]
#[get("/search?<page>&<q>&<sd>&<sf>")]
fn search(
    templates: State<Tera>,
    page: Option<u32>,
    q: Option<String>,
    sf: Option<String>,
    sd: Option<String>,
) -> Result<content::Html<String>, Error> {
    let params = Parameters::new(page, q, sf, sd);
    let images = api::search_images(&params)?;
    {
        #[derive(Serialize)]
        struct Context {
            params: Parameters,
            images: SearchImages,
        }
        let context = TeraContext::from_serialize(Context { params, images })?;
        let result = templates.render("search", &context)?;
        Ok(content::Html(result))
    }
}

// todo + tags
#[get("/images/<image_id>")]
fn image(image_id: u32) -> Result<String, Error> {
    let image = api::images(image_id)?;
    Ok("ok".into())
}

fn main() {
    dotenv().ok();

    let mut templates = Tera::default();
    templates
        .add_raw_templates(vec![
            ("base", include_str!("templates/base.html")),
            ("search", include_str!("templates/search.html")),
        ])
        .unwrap();

    Rocket::ignite()
        .mount("/", routes![index, search, image])
        .manage(templates)
        .launch();
}
