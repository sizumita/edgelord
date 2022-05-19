mod bucket;
mod route;

pub use route::Routes;

use serde::de::DeserializeOwned;
use worker::Method;
use edgelord::http::RequestBuilder;

pub struct HttpClient {
    bucket: dyn bucket::RateLimitBucket,
}

const BASE_URL: &'static str = "https://discord.com/api/v10";

impl HttpClient {
    async fn request<T>(&self, method: Method, route: Routes)
    where
        T: DeserializeOwned,
    {
        RequestBuilder::new(&*format!("{BASE_URL}{route}"))
            .method(method)
            .send()
            .await
            .unwrap()
            .json::<T>()
    }
}
