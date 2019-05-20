use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, DNT, HOST,
    PRAGMA, USER_AGENT,
};
use reqwest::{Error, Response};

pub struct Client {
    client: reqwest::Client,
    headers: HeaderMap,
}

impl Client {
    pub fn new(user_agent: &'static str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(user_agent));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, deflate, br"),
        );
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
        headers.insert(DNT, HeaderValue::from_static("1"));
        headers.insert(HOST, HeaderValue::from_static("api.arcsecond.io"));
        headers.insert(PRAGMA, HeaderValue::from_static("no-cache"));

        Self {
            headers,
            client: reqwest::Client::new(),
        }
    }

    pub fn request(&self, path: &str) -> Result<Response, Error> {
        println!("https://api.arcsecond.io{}", path);
        Ok(self
            .client
            .get(&format!("https://api.arcsecond.io{}", path))
            .headers(self.headers.clone())
            .send()?)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new("reqwest")
    }
}
