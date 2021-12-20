use crate::config::CONFIG;
use crate::models::{Parameters, SearchImages};

use reqwest::blocking::Client;
use reqwest::Proxy;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{Request, Response};

use std::io::Cursor;

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

// #[derive(Responder)]
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)] // need Responder + Debug
pub enum ApiError {
    NetError(reqwest::Error),
}

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
        // todo html
        let body = format!("{:?}", self);
        Response::build()
            .status(Status::InternalServerError)
            .sized_body(Cursor::new(body))
            .ok()
    }
}

/// Executes the search given by the q query parameter, and returns image responses.
pub fn search_images(params: &Parameters) -> Result<SearchImages, ApiError> {
    let url = format!("{}{}", &CONFIG.host, "/api/v1/json/search/images");
    match get(url, params) {
        Ok(text) => {
            // dbg!(&text);
            Ok(serde_json::from_str(text.as_str()).unwrap())
        }
        Err(e) => Err(ApiError::NetError(e)),
    }
}

fn get(url: String, params: &Parameters) -> Result<String, reqwest::Error> {
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
