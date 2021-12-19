use crate::config::CONFIG;
use crate::models::SearchImages;

use reqwest::blocking::Client;
use reqwest::Proxy;

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn search_images<T>(q: T) -> Result<SearchImages, reqwest::Error>
where
    T: Into<String>,
{
    let text = get_client()?
        .get(format!("{}{}", &CONFIG.host, "/api/v1/json/search/images"))
        .query(&[("key", &CONFIG.api_key), ("q", &q.into())])
        .send()?
        .text()?;
    Ok(serde_json::from_str(text.as_str()).unwrap())
}

#[inline(always)]
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
