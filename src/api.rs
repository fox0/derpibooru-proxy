use crate::config::CONFIG;
use crate::errors::Error;
use crate::models::{Image, Parameters, SearchImages};

use reqwest::blocking::Client;
use reqwest::Proxy;
use serde::Deserialize;

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Executes the search given by the q query parameter, and returns image responses.
pub fn search_images(params: &Parameters) -> Result<SearchImages, Error> {
    let url = format!("{}/api/v1/json/search/images", &CONFIG.host);
    let text = get_with_params(url, params)?;
    // dbg!(&text);
    let result = serde_json::from_str(text.as_str())?;
    Ok(result)
}

/// Fetches an image response for the image ID referenced by the `image_id` URL parameter.
pub fn images(image_id: u32) -> Result<Image, Error> {
    let url = format!("{}/api/v1/json/images/{}", &CONFIG.host, image_id);
    let text = get_with_params(url, &Parameters::new_only_key())?;
    {
        #[derive(Deserialize)]
        struct R {
            image: Image,
        }
        let result: R = serde_json::from_str(text.as_str())?;
        Ok(result.image)
    }
}

fn get_with_params(url: String, params: &Parameters) -> Result<String, reqwest::Error> {
    get_client()?.get(url).query(params).send()?.text()
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
