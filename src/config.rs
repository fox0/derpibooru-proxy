use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

const HOST: &str = "https://derpibooru.org";
const HOST_TOR: &str = "https://derpinxghr4jpjk4h4acjxyrx4rcwtk7ggjyt32uyaxgodqq7cfewuqd.onion";
const TOR: &str = "socks5h://127.0.0.1:9050";

/// Конфиг приложения
/// заполняется из переменных окружения `DERPI_KEY`, `DERPI_USE_TOR`. `https_proxy` и файла .env
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Config {
    pub api_key: String,
    pub host: String,
    pub https_proxy: Option<String>,
}

impl Config {
    fn new() -> Self {
        let api_key = env::var("DERPI_KEY").expect("you must set `DERPI_KEY` environment variable");
        let is_use_tor = env::var("DERPI_USE_TOR").unwrap_or_else(|_| "0".into()) == "1";
        let host = if is_use_tor { HOST_TOR } else { HOST };
        let host = host.to_string();
        let mut https_proxy = env::var("https_proxy").ok();
        if is_use_tor && https_proxy.is_none() {
            https_proxy = Some(TOR.into());
        }
        let config = Self {
            api_key,
            host,
            https_proxy,
        };
        #[cfg(debug_assertions)]
        dbg!(&config);
        config
    }
}
