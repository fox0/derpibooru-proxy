use crate::config::CONFIG;

use reqwest::blocking::Client;
use reqwest::Proxy;

pub fn search_images<T>(q: T) -> Result<String, reqwest::Error>
where
    T: Into<String>,
{
    get_client()?
        .get(format!("{}{}", &CONFIG.host, "/api/v1/json/search/images"))
        .query(&[("key", &CONFIG.api_key), ("q", &q.into())])
        .send()?
        .text()
}

#[inline(always)]
fn get_client() -> Result<Client, reqwest::Error> {
    // todo is_tor
    let mut cb = Client::builder().danger_accept_invalid_certs(true);
    if let Some(https_proxy) = &CONFIG.https_proxy {
        // todo move to lazy
        let proxy = Proxy::all(https_proxy)?;
        cb = cb.proxy(proxy);
    }
    cb.build()
}
