#![warn(clippy::pedantic)]

mod config;

use crate::config::CONFIG;

fn main() {
    // https_proxy=socks5h://127.0.0.1:9050
    // curl --insecure
    // /api/v1/json/comments/1000?key=KEY
    println!("Hello, world! {}", CONFIG.api_key);
}
