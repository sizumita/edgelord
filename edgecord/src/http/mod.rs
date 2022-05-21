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
    if #[cfg(target_arch = "wasm32")] {
        use worker::wasm_bindgen::JsValue;
        use edgelord::http::RequestBuilder;
    } else {
        use std::str::FromStr;
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

    #[cfg(not(target_arch = "wasm32"))]
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
        self._http
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
            .unwrap()
            .json::<T>()
            .await
            .map_err(
                |err| match err.status().unwrap_or(reqwest::StatusCode::BAD_REQUEST) {
                    reqwest::StatusCode::FORBIDDEN => Error::Forbidden,
                    reqwest::StatusCode::NOT_FOUND => Error::NotFound,
                    _ => Error::HttpError(err.to_string()),
                },
            )
    }
}
