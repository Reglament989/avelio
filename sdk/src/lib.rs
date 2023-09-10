use reqwest::{header, Client};

pub mod auth;
pub mod proto;
pub mod track;
pub(crate) mod utils;

#[derive(Debug)]
pub struct Avelio {
    pub c: Client,
    pub base_url: String,
    pub token: String,
}

impl Avelio {
    pub fn new(base_url: String, token: Option<String>) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/protobuf".parse().unwrap());
        Self {
            c: Client::builder().default_headers(headers).build().unwrap(),
            base_url,
            token: token.unwrap_or_default(),
        }
    }
}

impl Default for Avelio {
    fn default() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/protobuf".parse().unwrap());
        Self {
            c: Client::builder().default_headers(headers).build().unwrap(),
            base_url: String::from("http://localhost:56750"),
            token: String::from(""),
        }
    }
}
