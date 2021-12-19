#![feature(decl_macro)]
#![warn(clippy::pedantic)]

mod api;
mod config;
mod models;

use crate::api::search_images;

use dotenv::dotenv;
use rocket::response::content;
use rocket::{get, routes};

/// Главная страница
#[get("/")]
fn index() -> Result<content::Html<String>, reqwest::Error> {
    _images(1)
}

#[get("/images")]
fn images() -> Result<content::Html<String>, reqwest::Error> {
    _images(1)
}

#[get("/images?<page>")]
fn images_page(page: u32) -> Result<content::Html<String>, reqwest::Error> {
    _images(page)
}

fn _images(page: u32) -> Result<content::Html<String>, reqwest::Error> {
    let mut result = String::new();
    let ls = search_images("*", page)?;
    for i in ls.images {
        let title = i.get_title();
        result += format!(
            r#"<div class="thumb">
              <!-- todo upvotes -->
              <a href="/images/{}" title="{}">
                <img alt="{}" src="{}"/>
              </a>
            </div>"#,
            i.id, title, title, i.representations.thumb
        )
        .as_str();
    }
    let result = format!(
        r#"<!doctype html>
          <html>
          <head>
          <style>
          .thumb {{
            max-height: 250px;
            max-width: 250px;
            display: inline-block;
            margin: 0 6px 6px 0;
          }}
          </style>
          </head>
          <body>{}</body>
          </html>"#,
        result
    );
    Ok(content::Html(result))
}

#[get("/images/<id>")]
fn images_id(id: u32) -> Result<content::Html<String>, reqwest::Error> {
    todo!()
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .mount("/", routes![index, images, images_page, images_id])
        // todo custom error handler
        .launch();
}
