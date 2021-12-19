use crate::api::search_images;

use rocket::handler::Outcome;
use rocket::response::{content, Redirect};
use rocket::{Data, Request};

/// Главная страница
pub fn index<'r>(req: &'r Request, _: Data) -> Outcome<'r> {
    Outcome::from(req, Redirect::to("/images"))
}

pub fn images<'r>(req: &'r Request, _: Data) -> Outcome<'r> {
    let page: Option<Result<String, _>> = req.get_query_value("page");
    let page: u32 = match page {
        // не валидируем, так как лень
        Some(v) => v.unwrap().parse().unwrap(),
        None => 1,
    };

    let mut result = String::new();
    let ls = search_images("*", page).unwrap(); // todo
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
    Outcome::from(req, content::Html(result))
}

// #[get("/<name>/<age>")]
// fn hello(name: String, age: u8) -> String {
//     format!("Hello, {} year old named {}!", age, name)
// }
