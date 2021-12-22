use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
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
