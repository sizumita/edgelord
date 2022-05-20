mod bucket;
mod route;

pub use route::Routes;

use serde::de::DeserializeOwned;
use worker::Method;
use edgelord::http::RequestBuilder;
use crate::http::bucket::DefaultRateLimitBucket;

pub struct HttpClient {
    token: String,
    bucket: Box<dyn bucket::RateLimitBucket>,
    ua: String,
}

#[allow(dead_code)]
const BASE_URL: &'static str = "https://discord.com/api/v10";

impl HttpClient {
    pub fn new(token: &str) -> Self {
        Self {
            token: format!("Bot {token}"),
            bucket: Box::new(DefaultRateLimitBucket {}),
            ua: format!("Discord Bot (https://github.com/sizumita/edgelord 0.0.1)"),
        }
    }

    pub async fn request<T>(&self, method: Method, route: Routes)
    -> T where
        T: DeserializeOwned,
    {
        RequestBuilder::new(&*format!("{}{}", BASE_URL, route))
            .method(method)
            .header("Content-Type", "application/json")
            .header("Authorization", &*self.token)
            .header("User-Agent", &*self.ua)
            .send()
            .await
            .unwrap()
            .json::<T>()
            .await
            .unwrap()
    }
}
