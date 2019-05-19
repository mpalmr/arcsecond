use reqwest::{Error, Response};

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn request(&self, path: &str) -> Result<Response, Error> {
        Ok(self
            .client
            .get(&format!("https://api.arcsecond.io/{}", path))
            .send()?)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
