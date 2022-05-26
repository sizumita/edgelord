mod bucket;
mod route;

use cfg_if::cfg_if;
pub use route::Routes;

use crate::http::bucket::DefaultRateLimitBucket;
use crate::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use worker::Method;

cfg_if! {
    if #[cfg(all(not(target_arch = "wasm32"), feature = "local"))] {
        use std::str::FromStr;
    } else {
        use worker::wasm_bindgen::JsValue;
        use edgelord::http::RequestBuilder;
    }
}

pub struct HttpClient {
    token: String,
    _bucket: Box<dyn bucket::RateLimitBucket>,
    ua: String,
    #[cfg(not(target_arch = "wasm32"))]
    _http: reqwest::Client,
}

#[allow(dead_code)]
const BASE_URL: &str = "https://discord.com/api/v10";

impl HttpClient {
    pub fn new(token: &str) -> Self {
        Self {
            token: format!("Bot {token}"),
            _bucket: Box::new(DefaultRateLimitBucket {}),
            ua: "Discord Bot (https://github.com/sizumita/edgelord 0.0.1)".to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            _http: reqwest::Client::new(),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn request<T, B>(
        &self,
        method: Method,
        route: Routes,
        body: Option<B>,
    ) -> crate::Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let mut response = RequestBuilder::new(&*format!("{}{}", BASE_URL, route))
            .method(method)
            .header("Content-Type", "application/json")
            .header("Authorization", &*self.token)
            .header("User-Agent", &*self.ua)
            .body(body.and_then(|b| Some(JsValue::from(&*serde_json::to_string(&b).unwrap()))))
            .send()
            .await
            .unwrap();

        match response.status_code() {
            i if i < 399 => Ok(response.json::<T>().await.unwrap()),
            403 => Err(Error::Forbidden),
            404 => Err(Error::NotFound),
            // TODO: add errors
            _ => Err(Error::HttpError(response.text().await.unwrap())),
        }
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "local"))]
    pub async fn request<T, B>(
        &self,
        method: Method,
        route: Routes,
        body: Option<B>,
    ) -> crate::Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let response = self
            ._http
            .request(
                reqwest::Method::from_str(&*method.to_string()).unwrap(),
                &*format!("{}{}", BASE_URL, route),
            )
            .header("Content-Type", "application/json")
            .header("Authorization", &*self.token)
            .header("User-Agent", &*self.ua)
            .body(
                body.map(|x| serde_json::to_string(&x).unwrap())
                    .unwrap_or_default(),
            )
            .send()
            .await
            .unwrap();

        if response.status().clone().is_success() {
            return Ok(response.json::<T>().await.unwrap());
        }
        match response.status() {
            reqwest::StatusCode::FORBIDDEN => Err(Error::Forbidden),
            reqwest::StatusCode::NOT_FOUND => Err(Error::NotFound),
            _ => Err(Error::HttpError(response.text().await.unwrap_or_default())),
        }
    }
}
