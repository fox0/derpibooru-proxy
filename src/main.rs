#![feature(decl_macro)]
#![warn(clippy::pedantic)]

mod api;
mod config;
mod models;

use crate::api::search_images;

use dotenv::dotenv;
use rocket::response::content;
use rocket::{get, routes};

#[get("/")]
fn index() -> Result<content::Html<String>, reqwest::Error> {
    let mut result = String::new();
    let ls = search_images("*")?;
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
