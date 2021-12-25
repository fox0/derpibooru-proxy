use crate::errors::Error;
use crate::models::{SortDirection, SortField};

use lazy_static::lazy_static;
use rocket::response::content::Html;
use tera::{Context, Tera};

lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut result = Tera::default();
        result.add_raw_templates(RAW_TEMPLATES).unwrap();
        result
    };
}

const RAW_TEMPLATES: [(&str, &str); 4] = [
    ("main.css", include_str!("templates/main.css")),
    ("main.js", include_str!("templates/main.js")),
    ("base", include_str!("templates/base.html")),
    ("search", include_str!("templates/search.html")),
];

pub type Template = Html<String>;

/// ```
/// let mut context = Context::new();
/// context.insert("images", &images);
/// render("search", &mut context)
/// ```
pub fn render(template_name: &str, context: &mut Context) -> Result<Template, Error> {
    // add context_processor
    context.insert("sf", &SortField::get_choices());
    context.insert("sd", &SortDirection::get_choices());
    let result = TEMPLATES.render(template_name, context)?;
    Ok(Html(result))
}
