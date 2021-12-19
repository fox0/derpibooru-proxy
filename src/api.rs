use crate::config::CONFIG;
use crate::models::{Parameters, SearchImages};

use reqwest::blocking::Client;
use reqwest::Proxy;

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn search_images(params: &Parameters) -> Result<SearchImages, reqwest::Error> {
    let text = get_client()?
        .get(format!("{}{}", &CONFIG.host, "/api/v1/json/search/images"))
        .query(params)
        .send()?
        .text()?;
    // dbg!(&text);
    Ok(serde_json::from_str(text.as_str()).unwrap())
}

fn get_client() -> Result<Client, reqwest::Error> {
    let mut cb = Client::builder();
    if let Some(https_proxy) = &CONFIG.https_proxy {
        // todo move to lazy
        let proxy = Proxy::all(https_proxy)?;
        cb = cb.proxy(proxy);
    }
    cb.user_agent(APP_USER_AGENT)
        .danger_accept_invalid_certs(true) // todo is_tor
        .build()
}
