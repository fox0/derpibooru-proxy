use crate::config::CONFIG;
use crate::errors::Error;
use crate::models::{Parameters, SearchImages};

use reqwest::blocking::Client;
use reqwest::Proxy;
// use serde::Deserialize;
use lazy_static::lazy_static;
use std::time::Duration;

const TIMEOUT: Duration = Duration::new(120, 0);
const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

lazy_static! {
    static ref HTTP_CLIENT: Client = {
    let mut cb = Client::builder();
    if let Some(https_proxy) = &CONFIG.https_proxy {
        let proxy = Proxy::all(https_proxy).expect("invalid `https_proxy`");
        cb = cb.proxy(proxy);
    }
    cb.user_agent(APP_USER_AGENT)
        .timeout(TIMEOUT)
        .danger_accept_invalid_certs(true) // todo is_tor
        .build()
        .expect("error unknown")
    };
}

/// Executes the search given by the q query parameter, and returns image responses.
pub fn search_images(params: &Parameters) -> Result<SearchImages, Error> {
    let url = format!("{}/api/v1/json/search/images", &CONFIG.host);
    let json = HTTP_CLIENT.get(url).query(params).send()?.text()?;
    match serde_json::from_str(json.as_str()) {
        Ok(v) => Ok(v),
        Err(e) => Err(Error::Json((e, json))),
    }
}

// /// Fetches an image response for the image ID referenced by the `image_id` URL parameter.
// pub fn images(image_id: u32) -> Result<Image, Error> {
//     let url = format!("{}/api/v1/json/images/{}", &CONFIG.host, image_id);
//     let text = get_with_params(url, &Parameters::new_only_key())?;
//     {
//         #[derive(Deserialize)]
//         struct R {
//             image: Image,
//         }
//         let result: R = serde_json::from_str(text.as_str())?;
//         Ok(result.image)
//     }
// }
